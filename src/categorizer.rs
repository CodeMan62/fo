use std::fs;
use std::path::Path;

pub fn categorize_files(path: &str) {
    let categories = [
        ("Images", vec!["jpg", "png", "gif", "bmp", "svg"]),
        ("Documents", vec!["pdf", "docx", "txt", "xlsx"]),
        ("Videos", vec!["mp4", "avi", "mov", "mkv"]),
        ("Code", vec!["rs", "py", "js", "ts", "c", "cpp", "java", "go", "php"]),
    ];

    let path = Path::new(path);
    if path.is_dir() {
        for entry in fs::read_dir(path).expect("Unable to read directory") {
            if let Ok(entry) = entry {
                let file_path = entry.path();
                if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
                    let mut categorized = false;
                    for (category, extensions) in &categories {
                        if extensions.contains(&ext) {
                            println!("File {:?} categorized as {}", file_path, category);
                            categorized = true;
                            break;
                        }
                    }
                    if !categorized {
                        println!("File {:?} categorized as Others", file_path);
                    }
                }
            }
        }
    } else {
        println!("Provided path is not a directory.");
    }
}

