use crate::agents::{get_agents, Agent};
use crate::scanner::{scan_all_agents, DiscoveredSkill};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub children: Vec<FileEntry>,
}

const MAX_DEPTH: u32 = 10;

fn read_dir_tree(path: &std::path::Path, depth: u32) -> Vec<FileEntry> {
    if depth > MAX_DEPTH {
        return Vec::new();
    }

    let mut entries = Vec::new();
    let Ok(read_dir) = std::fs::read_dir(path) else {
        return entries;
    };

    let mut items: Vec<_> = read_dir.flatten().collect();
    items.sort_by_key(|e| e.file_name());

    for entry in items {
        let entry_path = entry.path();
        let name = entry
            .file_name()
            .to_string_lossy()
            .to_string();

        // Use symlink_metadata to detect symlinks without following them
        let metadata = match std::fs::symlink_metadata(&entry_path) {
            Ok(m) => m,
            Err(_) => continue,
        };

        let is_symlink = metadata.file_type().is_symlink();
        let is_dir = entry_path.is_dir();

        let children = if is_dir && !is_symlink {
            read_dir_tree(&entry_path, depth + 1)
        } else {
            Vec::new()
        };

        entries.push(FileEntry {
            name,
            path: entry_path,
            is_dir,
            children,
        });
    }

    entries
}

#[tauri::command]
pub fn list_agents() -> Vec<Agent> {
    get_agents()
}

#[tauri::command]
pub fn scan_skills() -> Vec<DiscoveredSkill> {
    let agents = get_agents();
    scan_all_agents(&agents)
}

#[tauri::command]
pub fn list_skill_files(path: String) -> Result<Vec<FileEntry>, String> {
    let dir = PathBuf::from(&path);
    if !dir.exists() || !dir.is_dir() {
        return Err(format!("Directory not found: {}", path));
    }
    Ok(read_dir_tree(&dir, 0))
}
