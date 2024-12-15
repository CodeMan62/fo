use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub struct FileEntry {
    pub path: PathBuf,
    pub category: String,
}

pub fn categorize_files_with_config(files: &[PathBuf], config: &HashMap<String, Vec<String>>) -> Vec<FileEntry> {
    files
        .iter()
        .filter_map(|file| {
            if let Some(ext) = file.extension() {
                let category = config
                    .iter()
                    .find_map(|(cat, exts)| {
                        if exts.contains(&ext.to_str().unwrap_or("").to_lowercase()) {
                            Some(cat.clone())
                        } else {
                            None
                        }
                    })
                    .unwrap_or_else(|| "Others".to_string());
                Some(FileEntry {
                    path: file.clone(),
                    category,
                })
            } else {
                None
            }
        })
        .collect()
}

