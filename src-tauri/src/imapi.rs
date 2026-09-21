use crate::commands::{BurnStatus, BurnerInfo, Track};
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;

#[derive(Debug, Deserialize)]
struct WindowsCdDrive {
    #[serde(rename = "DeviceID")]
    device_id: String,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Drive")]
    drive: Option<String>,
    #[serde(rename = "PNPDeviceID")]
    pnp_device_id: Option<String>,
}

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum ImapiError {
    #[error("COM initialization failed: {0}")]
    ComInit(String),
    #[error("No optical drive detected. Please connect a CD/DVD writer and make sure it appears in Windows Device Manager.")]
    NoBurnersFound,
    #[error("Burn operation failed: {0}")]
    BurnFailed(String),
    #[error("Device error: {0}")]
    DeviceError(String),
}

pub struct CdBurner;

impl CdBurner {
    pub fn new() -> Result<Self, ImapiError> {
        Ok(CdBurner)
    }

    pub async fn get_devices(&self) -> Result<Vec<BurnerInfo>, ImapiError> {
        #[cfg(target_os = "windows")]
        {
            detect_windows_cd_drives()
        }

        #[cfg(not(target_os = "windows"))]
        {
            Err(ImapiError::DeviceError(
                "CD burning only supported on Windows".to_string(),
            ))
        }
    }

    pub async fn burn_audio_cd(
        &self,
        burner_id: &str,
        tracks: &[Track],
        playlist_name: &str,
    ) -> Result<String, ImapiError> {
        if tracks.is_empty() {
            return Err(ImapiError::BurnFailed("No tracks provided".to_string()));
        }

        let available = self.get_devices().await?;
        if available.is_empty() {
            return Err(ImapiError::NoBurnersFound);
        }

        let resolved_burner_id = if burner_id.trim().is_empty() {
            available[0].id.clone()
        } else if available.iter().any(|drive| drive.id == burner_id) {
            burner_id.to_string()
        } else {
            available[0].id.clone()
        };

        let source_dir = prepare_track_directory(tracks, playlist_name)?;
        let burn_script = build_imapi_burn_script(&resolved_burner_id, &source_dir, playlist_name);

        #[cfg(target_os = "windows")]
        {
            let output = Command::new("powershell")
                .args([
                    "-NoProfile",
                    "-NonInteractive",
                    "-ExecutionPolicy",
                    "Bypass",
                    "-Command",
                    &burn_script,
                ])
                .output()
                .map_err(|err| {
                    ImapiError::BurnFailed(format!("Unable to launch Windows burn engine: {}", err))
                })?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                let stdout = String::from_utf8_lossy(&output.stdout);
                let details = stderr.trim();
                let combined = if details.is_empty() {
                    stdout.trim().to_string()
                } else {
                    details.to_string()
                };

                let _ = fs::remove_dir_all(&source_dir);
                return Err(ImapiError::BurnFailed(format!(
                    "Windows IMAPI burn failed: {}",
                    if combined.is_empty() {
                        "unknown error".to_string()
                    } else {
                        combined
                    }
                )));
            }

            let _ = fs::remove_dir_all(&source_dir);
            Ok(format!(
                "Burn started successfully on {} with {} track(s).",
                resolved_burner_id,
                tracks.len()
            ))
        }

        #[cfg(not(target_os = "windows"))]
        {
            let _ = fs::remove_dir_all(&source_dir);
            Err(ImapiError::BurnFailed(
                "Disc burning is only supported on Windows with IMAPI2.".to_string(),
            ))
        }
    }

    pub async fn get_status(&self, _burner_id: &str) -> Result<BurnStatus, ImapiError> {
        Ok(BurnStatus {
            progress: 0,
            status: "Ready".to_string(),
            remaining_time: 0,
        })
    }
}

fn sanitize_volume_name(input: &str) -> String {
    let sanitized = input
        .chars()
        .map(|ch| match ch {
            c if c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | ' ') => c,
            _ => ' ',
        })
        .collect::<String>();

    let normalized = sanitized.split_whitespace().collect::<Vec<_>>().join(" ");
    let truncated = normalized.chars().take(32).collect::<String>();
    if truncated.is_empty() {
        "MOMOCIDER".to_string()
    } else {
        truncated
    }
}

fn validate_local_track_source(track_path: &str, title: &str) -> Result<PathBuf, ImapiError> {
    let trimmed = track_path.trim();
    if trimmed.is_empty() {
        return Err(ImapiError::BurnFailed(format!(
            "Track '{}' has no valid local file path.",
            title
        )));
    }

    if trimmed.starts_with("http://") || trimmed.starts_with("https://") {
        return Err(ImapiError::BurnFailed(format!(
            "Track '{}' must be a local MP3 file before burning. Download or export it first, then try again.",
            title
        )));
    }

    let source = Path::new(trimmed);
    if !source.exists() {
        return Err(ImapiError::BurnFailed(format!(
            "Track '{}' is missing from disk: {}",
            title, trimmed
        )));
    }

    Ok(source.to_path_buf())
}

fn prepare_track_directory(tracks: &[Track], playlist_name: &str) -> Result<PathBuf, ImapiError> {
    let unique_suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or_default();
    let temp_root = std::env::temp_dir().join("momocider");
    let work_dir = temp_root.join(format!(
        "{}_{}",
        sanitize_volume_name(playlist_name),
        unique_suffix
    ));

    fs::create_dir_all(&work_dir).map_err(|err| {
        ImapiError::BurnFailed(format!(
            "Failed to create temporary burn directory: {}",
            err
        ))
    })?;

    for (index, track) in tracks.iter().enumerate() {
        let source = validate_local_track_source(&track.path, &track.title)?;
        let extension = source
            .extension()
            .and_then(|value| value.to_str())
            .filter(|value| !value.is_empty())
            .unwrap_or("wav");
        let target_path = work_dir.join(format!(
            "{:02}_{}.{}",
            index + 1,
            sanitize_track_title(&track.title),
            extension
        ));

        fs::copy(&source, &target_path).map_err(|err| {
            ImapiError::BurnFailed(format!(
                "Failed to copy track '{}' into the burn folder: {}",
                track.title, err
            ))
        })?;
    }

    Ok(work_dir)
}

fn sanitize_track_title(value: &str) -> String {
    let sanitized = value
        .chars()
        .map(|ch| match ch {
            c if c.is_ascii_alphanumeric() => c,
            ' ' | '-' | '_' => ch,
            _ => '_',
        })
        .collect::<String>();

    let trimmed = sanitized.trim();
    if trimmed.is_empty() {
        "track".to_string()
    } else {
        trimmed.to_string()
    }
}

fn build_imapi_burn_script(burner_id: &str, source_dir: &Path, playlist_name: &str) -> String {
    let burner_id = burner_id.replace("'", "''");
    let source_dir = source_dir.to_string_lossy().replace("'", "''");
    let volume_name = sanitize_volume_name(playlist_name).replace("'", "''");

    format!(
                "${{ErrorActionPreference}} = 'Stop'; \
$driveId = '{}'; \
$sourceDir = '{}'; \
$volumeName = '{}'; \
$mediaType = 2; \
$matchingDrives = Get-CimInstance -Class Win32_CDROMDrive | Where-Object {{ \
    $_.DeviceID -eq $driveId -or \
    $_.DeviceID -like \"*$driveId*\" -or \
    $_.PNPDeviceID -like \"*$driveId*\" -or \
    $driveId -like \"*$($_.DeviceID)*\" -or \
    $driveId -like \"*$($_.PNPDeviceID)*\" \
}}; \
$drive = $matchingDrives | Select-Object -First 1; \
if (-not $drive) {{ \
    $drive = Get-CimInstance -Class Win32_CDROMDrive | Select-Object -First 1; \
    if (-not $drive) {{ throw \"No optical drive detected for burn.\" }}; \
}}; \
$files = @(Get-ChildItem -LiteralPath $sourceDir -Filter '*.wav' | Sort-Object Name); \
if ($files.Count -eq 0) {{ throw \"No PCM WAV audio tracks were prepared.\" }}; \
            $recorderPath = if ($drive.Drive) {{ $drive.Drive }} else {{ $drive.DeviceID }}; \
            $discMaster = New-Object -ComObject IMAPI2.MsftDiscMaster2; \
            $recorderCandidates = @(); \
            try {{ \
                for ($index = 0; $index -lt $discMaster.Count; $index++) {{ \
                    $recorderCandidates += $discMaster.Item($index); \
                }}; \
            }} catch {{ }}; \
            $recorderCandidates += @($driveId, $drive.DeviceID, $drive.PNPDeviceID); \
            $recorderCandidates = $recorderCandidates | \
                Where-Object {{ $_ -and $_.ToString().Trim() }} | \
                Select-Object -Unique; \
            $recorder = New-Object -ComObject IMAPI2.MsftDiscRecorder2; \
            $initialized = $false; \
            foreach ($candidate in $recorderCandidates) {{ \
                try {{ \
                    $recorder.InitializeDiscRecorder($candidate.ToString()); \
                    $initialized = $true; \
                    break; \
                }} catch {{ }}; \
            }}; \
            if (-not $initialized) {{ \
                throw \"IMAPI could not initialize the optical recorder. Tried the Windows device, PNP, and drive identifiers for $recorderPath.\"; \
            }}; \
$format = New-Object -ComObject IMAPI2.MsftDiscFormat2TrackAtOnce; \
$format.Recorder = $recorder; \
$format.ClientName = 'Momocider'; \
$format.PrepareMedia(); \
Add-Type @' \
using System; \
using System.Runtime.InteropServices; \
public static class MomociderNativeStreams {{ \
    [DllImport(\"shlwapi.dll\", CharSet = CharSet.Unicode)] \
    public static extern int SHCreateStreamOnFileEx(string path, uint mode, uint attributes, bool create, IntPtr template, out IntPtr stream); \
}} \
'@; \
foreach ($file in $files) {{ \
    $streamPtr = [IntPtr]::Zero; \
    $hr = [MomociderNativeStreams]::SHCreateStreamOnFileEx($file.FullName, 0x20, 0, $false, [IntPtr]::Zero, [ref]$streamPtr); \
    if ($hr -ne 0) {{ throw \"Could not open WAV audio stream for $($file.FullName) (HRESULT $hr).\" }}; \
    $stream = [Runtime.InteropServices.Marshal]::GetObjectForIUnknown($streamPtr); \
    $format.AddAudioTrack($stream); \
    [Runtime.InteropServices.Marshal]::Release($streamPtr) | Out-Null; \
}}; \
$format.Finish(); \
Write-Output \"Successfully burned Audio CD '$volumeName' to $recorderPath\"",
        burner_id, source_dir, volume_name
    )
}

fn detect_windows_cd_drives() -> Result<Vec<BurnerInfo>, ImapiError> {
    let output = Command::new("powershell")
        .args([
            "-NoProfile",
            "-NonInteractive",
            "-ExecutionPolicy",
            "Bypass",
            "-Command",
            "Get-CimInstance Win32_CDROMDrive | Select-Object DeviceID, Name, Drive, PNPDeviceID | ConvertTo-Json -Compress",
        ])
        .output()
        .map_err(|err| {
            ImapiError::DeviceError(format!(
                "Failed to query Windows CD drives: {}",
                err
            ))
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(ImapiError::DeviceError(format!(
            "Windows optical drive query failed: {}",
            stderr.trim()
        )));
    }

    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if stdout.is_empty() {
        return Ok(vec![]);
    }

    let drives: Vec<WindowsCdDrive> = if stdout.starts_with('[') {
        serde_json::from_str(&stdout).map_err(|err| {
            ImapiError::DeviceError(format!("Failed to parse optical drive data: {}", err))
        })?
    } else {
        vec![serde_json::from_str(&stdout).map_err(|err| {
            ImapiError::DeviceError(format!(
                "Failed to parse single optical drive record: {}",
                err
            ))
        })?]
    };

    let burners = drives
        .into_iter()
        .filter_map(|drive| {
            let drive_name = drive
                .drive
                .clone()
                .filter(|value| !value.trim().is_empty())
                .unwrap_or_else(|| "Unknown drive".to_string());

            let sanitized_name = drive.name.trim();
            let pnp_id = drive
                .pnp_device_id
                .as_deref()
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(str::to_string);
            let device_id = drive.device_id.trim().to_string();
            let id_hint = if let Some(pnp) = &pnp_id {
                format!("{} | {}", device_id, pnp)
            } else {
                device_id.clone()
            };
            let burner_name = if sanitized_name.is_empty() {
                format!("CD/DVD Drive {} ({})", drive_name, id_hint)
            } else if pnp_id.is_some() {
                format!("{} ({}) [{}]", sanitized_name, drive_name, id_hint)
            } else {
                format!("{} ({}) [{}]", sanitized_name, drive_name, device_id)
            };

            if device_id.is_empty() {
                None
            } else {
                Some(BurnerInfo {
                    id: device_id.clone(),
                    name: burner_name,
                    supports_cd: true,
                    supports_dvd: true,
                })
            }
        })
        .collect::<Vec<_>>();

    if burners.is_empty() {
        Err(ImapiError::NoBurnersFound)
    } else {
        Ok(burners)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn burn_script_contains_drive_and_source_dir() {
        let dir = std::env::temp_dir().join("momocider-test");
        let script = build_imapi_burn_script("\\\\.\\CDROM0", &dir, "Momocider Vol 1");

        assert!(script.contains("CDROM0"));
        assert!(script.contains("momocider-test"));
        assert!(script.contains("IMAPI2.MsftDiscFormat2TrackAtOnce"));
        assert!(script.contains("$mediaType = 2"));
        assert!(script.contains("Filter '*.wav'"));
        assert!(script.contains("MsftDiscRecorder2"));
        assert!(script.contains("PrepareMedia()"));
        assert!(script.contains("SHCreateStreamOnFileEx"));
        assert!(script.contains("AddAudioTrack($stream)"));
        assert!(script.contains("Finish()"));
    }

    #[test]
    fn usb_storage_drive_ids_are_supported() {
        let burner_id =
            r"USBSTOR\CDROM&VEN_HP&PROD_DVDRAM_GT80N&REV_R102\HP_______DKXND5P83741_________&0";
        let script = build_imapi_burn_script(burner_id, &std::env::temp_dir(), "Momocider Vol 1");

        assert!(script.contains("USBSTOR"));
        assert!(script.contains("$driveId -like \"*$($_.PNPDeviceID)*\""));
    }

    #[test]
    fn track_title_and_volume_name_are_sanitized() {
        assert_eq!(
            sanitize_volume_name("Momocider / Vol. 1"),
            "Momocider Vol 1"
        );
        assert_eq!(sanitize_track_title("Song / Title: #1"), "Song _ Title_ _1");
    }

    #[test]
    fn rejects_remote_url_for_local_burn_source() {
        let err = validate_local_track_source(
            "https://www.youtube.com/watch?v=aqz-KE-bpKQ",
            "Big Buck Bunny",
        )
        .unwrap_err();

        assert!(err.to_string().contains("local MP3 file"));
    }
}
