use serde_json;
use std::{fs, path::Path};

use crate::model;

pub fn load_config(path: &str) -> Result<model::Config, Box<dyn std::error::Error>> {
    let fs_res = fs::read_to_string(path)?;
    let config: model::Config = serde_json::from_str(&fs_res)?;
    Ok(config)
}

pub fn get_config() -> Result<model::Config, Box<dyn std::error::Error>> {
    load_config("./config.json")
}

pub fn is_system_file(path: &str) -> bool {
    let sys_prefix = Path::new("C:\\Windows\\System32");
    let file_path = Path::new(path);
    file_path.starts_with(sys_prefix)
}

pub fn is_in_white_list(white_list: &Vec<String>, path: &str) -> bool {
    let path = Path::new(path);
    white_list.iter().any(|p| Path::new(p) == path)
}

pub fn convert_kb(kbs: usize) -> (usize, usize, usize) {
    let total_mb = kbs / 1024;
    let kb = kbs % 1024;
    let gb = total_mb / 1024;
    let mb = total_mb % 1024;
    (gb, mb, kb)
}
