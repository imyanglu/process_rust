use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    white_list: Vec<String>,
}
impl Config {
    pub fn is_in_white_list(&self, path: &str) -> bool {
        let path = Path::new(path).canonicalize().unwrap();
        let is_in_list = self.white_list.iter().any(|p| {
            let list_path = Path::new(p).canonicalize().unwrap();
            return path.starts_with(list_path);
        });

        is_in_list
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct IPRes {
    ip: String,
    country: String,
    country_code: String,
    region: String,
    zip_code: String,

    city: String,
    state_code: String,
    latitude: f32,
    longitude: f32,
    isp: String,
    isp_asn: usize,
    gdpr: bool,
}
