use std::fs;
use std::path::Path;

pub fn scan_files_in_dir(dir_path: &str) -> Vec<String> {
    let mut files = Vec::new();
    let path = Path::new(dir_path);

    if path.is_dir() {
        for entry in fs::read_dir(path).expect("Unable to read directory") {
            if let Ok(entry) = entry {
                let file_path = entry.path();
                if file_path.is_file() {
                    // Add file paths to the list
                    if let Some(path_str) = file_path.to_str() {
                        files.push(path_str.to_string());
                    }
                } else if file_path.is_dir() {
                    // Recursive call for subdirectories
                    let sub_files = scan_files_in_dir(file_path.to_str().unwrap());
                    files.extend(sub_files);
                }
            }
        }
    } else {
        println!("The provided path is not a directory.");
    }

    files
}

