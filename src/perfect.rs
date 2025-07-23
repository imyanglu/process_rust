use std::path::{Path, PathBuf};

pub fn scan_all_cache_directory() {
    let max_directory = 10;
    
    let root_directory = Path::new("C:\\").to_path_buf();
    let mut cache_directory: Vec<PathBuf> = Vec::new();
    print!("开始扫描");
    scan_cache_directory(root_directory, &mut cache_directory);
    println!("扫描结束--{:?}", cache_directory);
}
pub fn scan_cache_directory<'a>(directory: PathBuf, cache_path: &mut Vec<PathBuf>) {
    let is_cache_dir = directory.ends_with(".cache");
    if is_cache_dir {
        cache_path.push(directory);
    } else if directory.is_file() {
        return;
    } else {
        if let Ok(entries) = directory.read_dir() {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path_buf = entry.path();
                    scan_cache_directory(path_buf, cache_path);
                }
            }
        }
    }
}
