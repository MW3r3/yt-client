use std::process::{Command, Stdio};
use directories::UserDirs; // Ensure this is included

#[tauri::command]
fn download(id: &str) -> Result<String, String> {
    // Get the user's home directory
    let user_dirs = UserDirs::new().ok_or("Failed to find user directories")?;
    let download_path = user_dirs.home_dir().join("Videos");

    // Ensure the directory exists
    std::fs::create_dir_all(&download_path)
        .map_err(|e| format!("Failed to create directory: {}", e))?;

    // Set up the `yt-dlp` command without progress output
    let mut cmd = Command::new("yt-dlp")
        .arg(format!("https://www.youtube.com/watch?v={}", id))
        .arg("-o")
        .arg(format!("{}/%(title)s.%(ext)s", download_path.display()))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to start download: {}", e))?;

    // Wait for the command to complete without processing output
    let status = cmd.wait().map_err(|e| format!("Download process error: {}", e))?;
    if status.success() {
        Ok("Download completed successfully!".into())
    } else {
        Err("Download failed".into())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![download])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
