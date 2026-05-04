// 播放列表管理

use rodio::{Decoder, Source};
use std::io::BufReader;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use rayon::prelude::*;

use crate::defs::{is_supported_audio, MusicFile, Playlist};

/// 文件夹对话框结果
#[derive(Debug)]
pub enum FolderDialogResult {
    /// 用户选择了目录
    Selected(PathBuf),
    /// 用户取消了对话框
    Cancelled,
    /// 没有可用的图形对话框工具
    NoDialogAvailable,
}

/// 获取音频文件时长
fn get_audio_duration(path: &Path) -> Option<std::time::Duration> {
    let file = std::fs::File::open(path).ok()?;
    let source = Decoder::new(BufReader::new(file)).ok()?;
    source.total_duration()
}

/// 扫描目录中的音乐文件（并行获取时长）
pub fn scan_music_directory(dir: &Path) -> Result<Playlist, String> {
    if !dir.exists() {
        return Err(crate::langs::global_texts().fmt_dir_not_exist.replace("{}", &format!("{:?}", dir)));
    }

    if !dir.is_dir() {
        return Err(crate::langs::global_texts().fmt_not_a_dir.replace("{}", &format!("{:?}", dir)));
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
        return Err(crate::langs::global_texts().no_music_files_found.to_string());
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
pub fn open_folder_dialog() -> FolderDialogResult {
    use std::process::Command;

    // 使用 PowerShell 的文件夹选择对话框
    let output = match Command::new("powershell")
        .args([
            "-NoProfile",
            "-Command",
            "Add-Type -AssemblyName System.Windows.Forms; $folder = New-Object System.Windows.Forms.FolderBrowserDialog; if ($folder.ShowDialog() -eq 'OK') { $folder.SelectedPath }"
        ])
        .output()
    {
        Ok(o) => o,
        Err(_) => return FolderDialogResult::NoDialogAvailable,
    };

    if output.status.success() {
        let path = String::from_utf8_lossy(&output.stdout);
        let path = path.trim();
        if !path.is_empty() {
            return FolderDialogResult::Selected(PathBuf::from(path));
        }
    }

    // PowerShell 执行成功但用户取消了对话框
    FolderDialogResult::Cancelled
}

/// 在 macOS 上打开文件夹选择对话框
#[cfg(target_os = "macos")]
pub fn open_folder_dialog() -> FolderDialogResult {
    use std::process::Command;

    // 使用 AppleScript 的 choose folder 对话框
    let output = match Command::new("osascript")
        .args(&[
            "-e",
            &format!("POSIX path of (choose folder with prompt \"{}\")", crate::langs::global_texts().open_music_dir_title)
        ])
        .output()
    {
        Ok(o) => o,
        Err(_) => return FolderDialogResult::NoDialogAvailable,
    };

    if output.status.success() {
        let path = String::from_utf8_lossy(&output.stdout);
        let path = path.trim();
        if !path.is_empty() {
            return FolderDialogResult::Selected(PathBuf::from(path));
        }
    }

    // osascript 执行成功但用户取消了对话框（AppleScript 取消时返回非零退出码）
    FolderDialogResult::Cancelled
}

/// 在 Linux 上打开文件夹选择对话框
#[cfg(target_os = "linux")]
pub fn open_folder_dialog() -> FolderDialogResult {
    // 优先尝试 zenity（GNOME/XFCE 等桌面环境常见）
    match open_folder_zenity() {
        FolderDialogResult::Selected(path) => return FolderDialogResult::Selected(path),
        FolderDialogResult::Cancelled => return FolderDialogResult::Cancelled,
        FolderDialogResult::NoDialogAvailable => {}
    }

    // 尝试 kdialog（KDE 桌面环境）
    match open_folder_kdialog() {
        FolderDialogResult::Selected(path) => return FolderDialogResult::Selected(path),
        FolderDialogResult::Cancelled => return FolderDialogResult::Cancelled,
        FolderDialogResult::NoDialogAvailable => {}
    }

    // 尝试 yad（部分发行版默认提供）
    match open_folder_yad() {
        FolderDialogResult::Selected(path) => return FolderDialogResult::Selected(path),
        FolderDialogResult::Cancelled => return FolderDialogResult::Cancelled,
        FolderDialogResult::NoDialogAvailable => {}
    }

    // 尝试 qarma（部分发行版默认提供）
    match open_folder_qarma() {
        FolderDialogResult::Selected(path) => return FolderDialogResult::Selected(path),
        FolderDialogResult::Cancelled => return FolderDialogResult::Cancelled,
        FolderDialogResult::NoDialogAvailable => {}
    }

    // 尝试 python tkinter
    match open_folder_python_tk() {
        FolderDialogResult::Selected(path) => return FolderDialogResult::Selected(path),
        FolderDialogResult::Cancelled => return FolderDialogResult::Cancelled,
        FolderDialogResult::NoDialogAvailable => {}
    }

    // 尝试 python + PyQt5 文件对话框
    match open_folder_python_qt() {
        FolderDialogResult::Selected(path) => return FolderDialogResult::Selected(path),
        FolderDialogResult::Cancelled => return FolderDialogResult::Cancelled,
        FolderDialogResult::NoDialogAvailable => {}
    }

    // 尝试 python + PySide2（Deepin 等 Qt 桌面环境通常可用）
    match open_folder_python_pyside2() {
        FolderDialogResult::Selected(path) => return FolderDialogResult::Selected(path),
        FolderDialogResult::Cancelled => return FolderDialogResult::Cancelled,
        FolderDialogResult::NoDialogAvailable => {}
    }

    // 尝试 python + GTK3（部分桌面环境可用）
    match open_folder_python_gtk() {
        FolderDialogResult::Selected(path) => return FolderDialogResult::Selected(path),
        FolderDialogResult::Cancelled => return FolderDialogResult::Cancelled,
        FolderDialogResult::NoDialogAvailable => {}
    }

    FolderDialogResult::NoDialogAvailable
}

#[cfg(target_os = "linux")]
fn open_folder_zenity() -> FolderDialogResult {
    use std::process::Command;

    let output = match Command::new("zenity")
        .args(&[
            "--file-selection",
            "--directory",
            &format!("--title={}", crate::langs::global_texts().open_music_dir_title),
        ])
        .output()
    {
        Ok(o) => o,
        Err(_) => return FolderDialogResult::NoDialogAvailable,
    };

    if output.status.success() {
        let path = String::from_utf8_lossy(&output.stdout);
        let path = path.trim();
        if !path.is_empty() {
            return FolderDialogResult::Selected(PathBuf::from(path));
        }
    }
    // zenity 存在但用户取消了对话框（退出码非0表示取消）
    // 区分：如果 stderr 包含 "No such file" 等说明是工具问题，否则是用户取消
    // zenity 取消时退出码为 1，且无 stdout 输出
    let stderr = String::from_utf8_lossy(&output.stderr);
    if stderr.contains("not found") || stderr.contains("No such file") || stderr.contains("command not found") {
        return FolderDialogResult::NoDialogAvailable;
    }
    FolderDialogResult::Cancelled
}

#[cfg(target_os = "linux")]
fn open_folder_kdialog() -> FolderDialogResult {
    use std::process::Command;

    let output = match Command::new("kdialog")
        .args(&[
            "--getexistingdirectory",
            &format!("--title={}", crate::langs::global_texts().open_music_dir_title),
        ])
        .output()
    {
        Ok(o) => o,
        Err(_) => return FolderDialogResult::NoDialogAvailable,
    };

    if output.status.success() {
        let path = String::from_utf8_lossy(&output.stdout);
        let path = path.trim();
        if !path.is_empty() {
            return FolderDialogResult::Selected(PathBuf::from(path));
        }
    }
    // kdialog 取消时退出码为 1，通常输出为空
    FolderDialogResult::Cancelled
}

#[cfg(target_os = "linux")]
fn open_folder_yad() -> FolderDialogResult {
    use std::process::Command;

    let output = match Command::new("yad")
        .args(&[
            "--file-selection",
            "--directory",
            &format!("--title={}", crate::langs::global_texts().open_music_dir_title),
        ])
        .output()
    {
        Ok(o) => o,
        Err(_) => return FolderDialogResult::NoDialogAvailable,
    };

    if output.status.success() {
        let path = String::from_utf8_lossy(&output.stdout);
        let path = path.trim();
        if !path.is_empty() {
            return FolderDialogResult::Selected(PathBuf::from(path));
        }
    }
    FolderDialogResult::Cancelled
}

#[cfg(target_os = "linux")]
fn open_folder_qarma() -> FolderDialogResult {
    use std::process::Command;

    let output = match Command::new("qarma")
        .args(&[
            "--file-selection",
            "--directory",
            &format!("--title={}", crate::langs::global_texts().open_music_dir_title),
        ])
        .output()
    {
        Ok(o) => o,
        Err(_) => return FolderDialogResult::NoDialogAvailable,
    };

    if output.status.success() {
        let path = String::from_utf8_lossy(&output.stdout);
        let path = path.trim();
        if !path.is_empty() {
            return FolderDialogResult::Selected(PathBuf::from(path));
        }
    }
    FolderDialogResult::Cancelled
}

#[cfg(target_os = "linux")]
fn open_folder_python_tk() -> FolderDialogResult {
    let script = format!(r#"import tkinter as tk
from tkinter import filedialog
root = tk.Tk()
root.withdraw()
root.attributes('-topmost', True)
path = filedialog.askdirectory(title='{}')
print(path if path else '')
"#, crate::langs::global_texts().open_music_dir_title);

    // 先尝试 python3，再尝试 python
    for python_cmd in &["python3", "python"] {
        if let Some(result) = try_python_script(python_cmd, &script) {
            return result;
        }
    }
    FolderDialogResult::NoDialogAvailable
}

#[cfg(target_os = "linux")]
fn open_folder_python_qt() -> FolderDialogResult {
    let script = format!(r#"import sys
try:
    from PyQt5.QtWidgets import QApplication, QFileDialog
except Exception:
    sys.exit(1)

app = QApplication.instance() or QApplication(sys.argv)
path = QFileDialog.getExistingDirectory(None, '{}')
print(path if path else '')
"#, crate::langs::global_texts().open_music_dir_title);

    for python_cmd in &["python3", "python"] {
        if let Some(result) = try_python_script(python_cmd, &script) {
            return result;
        }
    }
    FolderDialogResult::NoDialogAvailable
}

#[cfg(target_os = "linux")]
fn open_folder_python_pyside2() -> FolderDialogResult {
    let script = format!(r#"import sys
try:
    from PySide2.QtWidgets import QApplication, QFileDialog
except Exception:
    sys.exit(1)

app = QApplication.instance() or QApplication(sys.argv)
path = QFileDialog.getExistingDirectory(None, '{}')
print(path if path else '')
"#, crate::langs::global_texts().open_music_dir_title);

    for python_cmd in &["python3", "python"] {
        if let Some(result) = try_python_script(python_cmd, &script) {
            return result;
        }
    }
    FolderDialogResult::NoDialogAvailable
}

#[cfg(target_os = "linux")]
fn open_folder_python_gtk() -> FolderDialogResult {
    let script = format!(r#"import sys
try:
    import gi
    gi.require_version('Gtk', '3.0')
    from gi.repository import Gtk
except Exception:
    sys.exit(1)

Gtk.init_check()
dialog = Gtk.FileChooserDialog(
    title='{}',
    action=Gtk.FileChooserAction.SELECT_FOLDER,
    buttons=(Gtk.STOCK_CANCEL, Gtk.ResponseType.CANCEL, Gtk.STOCK_OPEN, Gtk.ResponseType.OK)
)
dialog.set_local_only(True)
response = dialog.run()
if response == Gtk.ResponseType.OK:
    path = dialog.get_filename()
    print(path if path else '')
else:
    print('')
dialog.destroy()
"#, crate::langs::global_texts().open_music_dir_title);

    for python_cmd in &["python3", "python"] {
        if let Some(result) = try_python_script(python_cmd, &script) {
            return result;
        }
    }
    FolderDialogResult::NoDialogAvailable
}

/// 尝试用指定的 python 命令执行脚本，返回对话框结果
#[cfg(target_os = "linux")]
fn try_python_script(python_cmd: &str, script: &str) -> Option<FolderDialogResult> {
    use std::process::Command;

    let output = match Command::new(python_cmd)
        .args(["-c", script])
        .output()
    {
        Ok(o) => o,
        Err(_) => return None,
    };

    if output.status.success() {
        let path = String::from_utf8_lossy(&output.stdout);
        let path = path.trim();
        if !path.is_empty() {
            return Some(FolderDialogResult::Selected(PathBuf::from(path)));
        }
        // 执行成功但用户取消了对话框（空输出表示取消）
        return Some(FolderDialogResult::Cancelled);
    }
    // 执行失败（可能是 GUI 库不可用）
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
