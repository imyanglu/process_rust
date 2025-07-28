use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

use tokio::fs;

pub async fn search_file(search: &str, path: &Path, file_extensions: HashSet<&str>) {
    println!("开始扫描文件");

    let mut exist_file_path: Vec<String> = Vec::new();
    scan_file(path.to_path_buf(), &mut exist_file_path, &file_extensions);
    println!("开始读取文件,文件数量{}", exist_file_path.len());
    for file_path in exist_file_path {
        let file_content = fs::read_to_string(&file_path).await.unwrap();
        if file_content.contains(search) {
            println!("{}: {}", file_path, search);
        }
    }
}
pub fn scan_file<'a>(
    directory: PathBuf,
    file_path: &mut Vec<String>,
    file_extensions: &HashSet<&str>,
) {
    if directory.is_file() {
        let extension = directory.extension().and_then(|s| s.to_str());
        if let Some(extension) = extension {
            if file_extensions.contains(&extension) {
                file_path.push(directory.to_string_lossy().to_string());
            }
        }
    } else {
        if let Ok(entries) = directory.read_dir() {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path_buf = entry.path();
                    scan_file(path_buf, file_path, file_extensions);
                }
            }
        }
    }
}
