use crate::categorizer::{categorize_files_with_config, FileEntry};
use crate::file_scaner::scan_files_recursively;
use crate::config::Config;
use std::fs;
use std::path::Path;

pub fn organize_files(root: &str, config: &Config, dry_run: bool) -> Result<(), Box<dyn std::error::Error>> {
    let root_path = Path::new(root);
    if !root_path.is_dir() {
        return Err("Provided path is not a directory".into());
    }

    let files = scan_files_recursively(root_path);
    let categorized_files = categorize_files_with_config(&files, &config.categories);

    for entry in categorized_files {
        let category_dir = root_path.join(&entry.category);
        if !dry_run {
            fs::create_dir_all(&category_dir)?;
            fs::rename(&entry.path, category_dir.join(entry.path.file_name().unwrap()))?;
        }
        println!(
            "{} -> {}",
            entry.path.display(),
            category_dir.display()
        );
    }

    Ok(())
}

