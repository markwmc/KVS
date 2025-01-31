use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde::{Serialize, Deserialize};

const DB_FILE: &str = "db.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct KVStore {
    pub data: HashMap<String, String>,
}

impl KVStore {
    pub fn new() -> Self {
        if Path::new(DB_FILE).exists() {
            let content = fs::read_to_string(DB_FILE).unwrap_or_else(|_| "{}".to_string());
            let data: HashMap<String, String> = serde_json::from_str(&content).unwrap_or_default();
            KVStore { data }
        } else {
            KVStore { data: HashMap::new() }
        }
    }

    pub fn save(&self) {
        let content = serde_json::to_string_pretty(&self.data).expect("Failed to serialize data");
        fs::write(DB_FILE, content).expect("Failed to write data to file");
    }
    pub fn set(&mut self, key: String, value: String) {
        self.data.insert(key,value);
        self.save();
    }
    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    pub fn delete(&mut self, key: &str) {
        self.data.remove(key);
        self.save();
    }

    pub fn list(&self) {
        for (key, value) in &self.data {
            println!("{}: {}", key, value);
        }
    }
}