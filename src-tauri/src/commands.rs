use crate::imapi::CdBurner;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BurnerInfo {
    pub id: String,
    pub name: String,
    pub supports_dvd: bool,
    pub supports_cd: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Track {
    pub path: String,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BurnStatus {
    pub progress: u32,
    pub status: String,
    pub remaining_time: u32,
}

#[tauri::command]
pub async fn get_burners() -> Result<Vec<BurnerInfo>, String> {
    let burner = CdBurner::new().map_err(|e| format!("Failed to initialize CD burner: {}", e))?;

    burner
        .get_devices()
        .await
        .map_err(|e| format!("Failed to get burners: {}", e))
}

#[tauri::command]
pub async fn burn_cd(
    burner_id: String,
    tracks: Vec<Track>,
    playlist_name: String,
) -> Result<String, String> {
    let burner = CdBurner::new().map_err(|e| format!("Failed to initialize CD burner: {}", e))?;

    burner
        .burn_audio_cd(&burner_id, &tracks, &playlist_name)
        .await
        .map_err(|e| format!("Burn failed: {}", e))
}

#[tauri::command]
pub async fn get_burn_status(burner_id: String) -> Result<BurnStatus, String> {
    let burner = CdBurner::new().map_err(|e| format!("Failed to initialize CD burner: {}", e))?;

    burner
        .get_status(&burner_id)
        .await
        .map_err(|e| format!("Failed to get status: {}", e))
}

#[tauri::command]
pub async fn fetch_track_info(url: String) -> Result<serde_json::Value, String> {
    // Call the Node backend
    let client = reqwest::Client::new();
    let response = client
        .post("http://localhost:5000/api/track/info")
        .json(&serde_json::json!({ "url": url }))
        .send()
        .await
        .map_err(|e| {
            format!(
                "Failed to reach backend: {}. Ensure Node server is running on port 5000",
                e
            )
        })?;

    response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))
}

#[tauri::command]
pub async fn export_zip(
    tracks: Vec<serde_json::Value>,
    playlist_name: String,
) -> Result<String, String> {
    // This calls the Node backend
    let client = reqwest::Client::new();
    let _response = client
        .post("http://localhost:5000/api/export-zip")
        .json(&serde_json::json!({ "tracks": tracks, "playlistName": playlist_name }))
        .send()
        .await
        .map_err(|e| format!("Export failed: {}", e))?;

    Ok("Export started. Files will be prepared for burning.".to_string())
}
