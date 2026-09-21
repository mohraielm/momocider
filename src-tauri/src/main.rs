#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod imapi;

use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::Duration;

fn main() {
    run();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|_app| {
            // Start Node backend subprocess on app startup
            std::thread::spawn(|| {
                let _ = start_node_backend();
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_burners,
            commands::burn_cd,
            commands::get_burn_status,
            commands::fetch_track_info,
            commands::export_zip,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn start_node_backend() -> std::io::Result<()> {
    // Give the Node process time to initialize
    std::thread::sleep(Duration::from_millis(500));

    // Try to find Node executable and server.js
    let (node_exe, server_path) = find_backend_files()?;
    let backend_dir = server_path
        .parent()
        .unwrap_or_else(|| std::path::Path::new("."))
        .to_path_buf();

    // Start Node process from the backend directory so relative imports work correctly.
    let _child = Command::new(&node_exe)
        .current_dir(&backend_dir)
        .arg(&server_path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;

    // Give Node time to start listening
    std::thread::sleep(Duration::from_secs(2));

    Ok(())
}

fn find_backend_files() -> std::io::Result<(String, PathBuf)> {
    let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap_or_else(|| std::path::Path::new("."))
        .to_path_buf();

    let backend_dir = workspace_root.join("backend");
    let server_path = backend_dir.join("server.js");

    if !server_path.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("backend/server.js not found at {}", server_path.display()),
        ));
    }

    let node_exe = find_node_executable()?;
    Ok((node_exe, server_path))
}

fn find_node_executable() -> std::io::Result<String> {
    let home_dir = std::env::var("USERPROFILE").unwrap_or_default();
    let mut candidates = vec![
        "node".to_string(),
        "node.exe".to_string(),
        format!(
            "{}\\AppData\\Local\\Microsoft\\WinGet\\Links\\node.exe",
            home_dir
        ),
        "C:\\Program Files\\nodejs\\node.exe".to_string(),
        "C:\\Program Files (x86)\\nodejs\\node.exe".to_string(),
    ];

    if let Ok(path_var) = std::env::var("PATH") {
        for part in path_var.split(';') {
            let trimmed = part.trim();
            if trimmed.is_empty() {
                continue;
            }
            let candidate = PathBuf::from(trimmed).join("node.exe");
            if candidate.exists() {
                candidates.insert(0, candidate.to_string_lossy().to_string());
            }
            let candidate = PathBuf::from(trimmed).join("node");
            if candidate.exists() {
                candidates.insert(0, candidate.to_string_lossy().to_string());
            }
        }
    }

    for candidate in candidates {
        if test_command(&candidate) {
            return Ok(candidate);
        }
    }

    Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "Node.js executable not found in PATH or common locations",
    ))
}

fn test_command(cmd: &str) -> bool {
    Command::new(cmd)
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}
