mod error;

use std::collections::HashMap;

#[derive(Default)]
pub struct KvStore {
    pub data: HashMap<String, String>,
}

enum KvsCommand {
    Set { key: String, value: String },
    Get { key: String },
    Rm { key: String },
}

impl KvStore {
    pub fn new() -> Self {
        KvStore {
            data: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, val: String) {
        self.data.insert(key, val);
    }

    pub fn get(&mut self, key: String) -> Option<String> {
        self.data.get(&key).map(|s| s.clone())
    }

    pub fn remove(&mut self, key: String) {
        self.data.remove(&key);
    }
}
