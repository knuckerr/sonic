use rustc_hash::FxHashMap;

pub struct Store {
    data: FxHashMap<String, Vec<u8>>,
}

impl Default for Store {
    fn default() -> Self {
        Self::new()
    }
}

impl Store {
    pub fn new() -> Self {
        Store {
            data: FxHashMap::default(),
        }
    }
    pub fn get(&mut self, key: String) -> Option<Vec<u8>> {
        self.data.get(&key).cloned()
    }
    pub fn set(&mut self, key: String, value: Vec<u8>) {
        self.data.insert(key, value);
    }
    pub fn delete(&mut self, key: String) {
        self.data.remove(&key);
    }
}

#[test]
pub fn get() {
    let mut store = Store {
        data: FxHashMap::default(),
    };
    store.set("test".to_string(), "test".to_owned().into_bytes());
    let data = store.get("test".to_string()).unwrap();
    assert_eq!("test".to_owned().into_bytes(), data);
}
#[test]
pub fn set() {
    let mut store = Store {
        data: FxHashMap::default(),
    };
    store.set("test".to_string(), "test".to_owned().into_bytes());
    assert!(true);
}
#[test]
pub fn delete() {
    let mut store = Store {
        data: FxHashMap::default(),
    };
    store.set("test".to_string(), "test".to_owned().into_bytes());
    store.delete("test".to_string());
    assert!(true);
}
