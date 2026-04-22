// 播放列表管理

use rodio::{Decoder, Source};
use std::io::BufReader;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use rayon::prelude::*;

use crate::defs::{is_supported_audio, MusicFile, Playlist};

/// 获取音频文件时长
fn get_audio_duration(path: &Path) -> Option<std::time::Duration> {
    let file = std::fs::File::open(path).ok()?;
    let source = Decoder::new(BufReader::new(file)).ok()?;
    source.total_duration()
}

/// 扫描目录中的音乐文件（并行获取时长）
pub fn scan_music_directory(dir: &Path) -> Result<Playlist, String> {
    if !dir.exists() {
        return Err(format!("目录不存在: {:?}", dir));
    }

    if !dir.is_dir() {
        return Err(format!("不是目录: {:?}", dir));
    }

    // 先收集所有音频文件路径
    let audio_paths: Vec<PathBuf> = WalkDir::new(dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file() && is_supported_audio(e.path()))
        .map(|e| e.path().to_path_buf())
        .collect();

    if audio_paths.is_empty() {
        return Err("未找到支持的音乐文件".to_string());
    }

    // 并行获取时长并创建 MusicFile
    let mut files: Vec<MusicFile> = audio_paths
        .par_iter()
        .map(|path| {
            let duration = get_audio_duration(path);
            MusicFile::with_duration(path.clone(), duration)
        })
        .collect();

    // 按文件名排序
    files.sort_by(|a, b| a.name.cmp(&b.name));

    let mut playlist = Playlist::new();
    playlist.files = files;
    playlist.directory = Some(dir.to_string_lossy().to_string());

    Ok(playlist)
}

/// 在 Windows 上打开文件夹选择对话框
#[cfg(target_os = "windows")]
pub fn open_folder_dialog() -> Option<PathBuf> {
    use std::process::Command;

    // 使用 PowerShell 的文件夹选择对话框
    let output = Command::new("powershell")
        .args([
            "-NoProfile",
            "-Command",
            "Add-Type -AssemblyName System.Windows.Forms; $folder = New-Object System.Windows.Forms.FolderBrowserDialog; if ($folder.ShowDialog() -eq 'OK') { $folder.SelectedPath }"
        ])
        .output()
        .ok()?;

    if output.status.success() {
        let path = String::from_utf8_lossy(&output.stdout);
        let path = path.trim();
        if !path.is_empty() {
            return Some(PathBuf::from(path));
        }
    }

    None
}

/// 在 macOS 上打开文件夹选择对话框
#[cfg(target_os = "macos")]
pub fn open_folder_dialog() -> Option<PathBuf> {
    use std::process::Command;

    // 使用 AppleScript 的 choose folder 对话框
    let output = Command::new("osascript")
        .args(&[
            "-e",
            "POSIX path of (choose folder with prompt \"打开音乐目录\")"
        ])
        .output()
        .ok()?;

    if output.status.success() {
        let path = String::from_utf8_lossy(&output.stdout);
        let path = path.trim();
        if !path.is_empty() {
            return Some(PathBuf::from(path));
        }
    }

    None
}

/// 在 Linux 上打开文件夹选择对话框
#[cfg(target_os = "linux")]
pub fn open_folder_dialog() -> Option<PathBuf> {
    use std::process::Command;

    // 优先尝试 zenity（GNOME/XFCE 等桌面环境常见）
    if let Some(path) = open_folder_zenity() {
        return Some(path);
    }

    // 尝试 kdialog（KDE 桌面环境）
    if let Some(path) = open_folder_kdialog() {
        return Some(path);
    }

    None
}

#[cfg(target_os = "linux")]
fn open_folder_zenity() -> Option<PathBuf> {
    use std::process::Command;

    let output = Command::new("zenity")
        .args(&[
            "--file-selection",
            "--directory",
            "--title=打开音乐目录",
        ])
        .output()
        .ok()?;

    if output.status.success() {
        let path = String::from_utf8_lossy(&output.stdout);
        let path = path.trim();
        if !path.is_empty() {
            return Some(PathBuf::from(path));
        }
    }

    None
}

#[cfg(target_os = "linux")]
fn open_folder_kdialog() -> Option<PathBuf> {
    use std::process::Command;

    let output = Command::new("kdialog")
        .args(&[
            "--getexistingdirectory",
            "--title=打开音乐目录",
        ])
        .output()
        .ok()?;

    if output.status.success() {
        let path = String::from_utf8_lossy(&output.stdout);
        let path = path.trim();
        if !path.is_empty() {
            return Some(PathBuf::from(path));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_playlist_new() {
        let playlist = Playlist::new();
        assert!(playlist.is_empty());
        assert_eq!(playlist.len(), 0);
    }

    #[test]
    fn test_playlist_add() {
        let mut playlist = Playlist::new();
        let file = MusicFile::new(PathBuf::from("test.mp3"));
        playlist.add(file);
        assert_eq!(playlist.len(), 1);
    }
}
