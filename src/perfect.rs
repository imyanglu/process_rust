use std::{
    path::{Path, PathBuf},
    sync::{Arc, Mutex, atomic::AtomicU64},
    thread,
    time::Instant,
};

use tokio::fs;

static dir_count: AtomicU64 = AtomicU64::new(0);
pub fn scan_dir(search: &str, path: &Path, file_extensions: &[&str]) {
    let mut exist_file_path: Vec<String> = Vec::new();
    let root_directory = Path::new("C:\\").to_path_buf();
    let a = scan_file(
        root_directory,
        &mut exist_file_path,
        file_extensions,
        search,
    )
    .await;
}
pub async fn scan_file<'a>(
    directory: PathBuf,
    file_path: &mut Vec<String>,
    file_extensions: &[&str],
    search: &'a str,
) {
    dir_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    if (directory.is_file()) {
        let extension = directory.extension().and_then(|s| s.to_str());
        if let Some(extension) = extension {
            if file_extensions.contains(&extension) {
                let file_content = fs::read_to_string(&directory).await.unwrap();
                if file_content.contains(search) {
                    file_path.push(directory.to_string_lossy().to_string());
                }
            }
        }
    } else {
        if let Ok(entries) = directory.read_dir() {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path_buf = entry.path();
                    scan_file(path_buf, file_path, file_extensions, search).await;
                }
            }
        }
    }
}

pub fn scan_all_cache_directory() {
    let max_directory = 10;

    let root_directory = Path::new("C:\\").to_path_buf();
    let cache_directory = Arc::new(Mutex::new(Vec::<PathBuf>::new()));

    let cache_cloned = Arc::clone(&cache_directory);
    print!("开始扫描");
    let start = Instant::now();
    let handle = thread::spawn(move || {
        scan_cache_directory(root_directory, &cache_cloned);
    });
    handle.join().unwrap();
    let end = Instant::now();
    let duration = end - start;
    let locked = cache_directory.lock().unwrap();
    println!(
        "扫描结束,用时{}s,找到缓存目录{}个",
        duration.as_secs_f32(),
        locked.len()
    );
}
pub fn scan_cache_directory<'a>(directory: PathBuf, cache_path: &Arc<Mutex<Vec<PathBuf>>>) {
    dir_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

    let is_cache_dir = directory.ends_with(".cache");
    if is_cache_dir {
        let mut vec = cache_path.lock().unwrap();
        vec.push(directory);
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
