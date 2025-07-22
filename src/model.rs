use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    white_list: Vec<String>,
}
impl Config {
    fn white_list(&self) -> &Vec<String> {
        &self.white_list
    }
    fn is_in_white_list(&self, name: &str) -> bool {
        self.white_list().contains(&name.to_string())
    }
}
