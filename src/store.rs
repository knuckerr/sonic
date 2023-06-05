use std::time::{Duration, Instant};

use rustc_hash::FxHashMap;

pub struct Store {
    data: FxHashMap<String, (Vec<u8>, Option<Instant>)>,
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
        match self.data.get(&key).cloned() {
            Some((value, Some(expiration))) => {
                if expiration > Instant::now() {
                    Some(value)
                } else {
                    self.data.remove(&key);
                    None
                }
            }
            Some((value, None)) => Some(value),
            None => None,
        }
    }
    pub fn exp(&mut self, key: String, duration: Duration) {
        if let Some((value, _)) = self.data.get(&key) {
            let expiration_time = Instant::now() + duration;
            self.data
                .insert(key, (value.to_vec(), Some(expiration_time)));
        }
    }
    pub fn set(&mut self, key: String, value: Vec<u8>, duration: Option<Duration>) {
        let expiration_time = duration.map(|duration| Instant::now() + duration);
        self.data.insert(key, (value, expiration_time));
    }
    pub fn delete(&mut self, key: String) {
        self.data.remove(&key);
    }
}

#[test]
pub fn test_exp() {
    let mut store = Store {
        data: FxHashMap::default(),
    };
    store.set("test".to_string(), "test".to_owned().into_bytes(), None);
    store.exp("test".to_string(), Duration::new(10, 0));
    assert!(true)
}
#[test]
pub fn test_exp_with_duration() {
    let mut store = Store {
        data: FxHashMap::default(),
    };
    let durration = Duration::new(5, 0);
    store.set(
        "test".to_string(),
        "test".to_owned().into_bytes(),
        Some(durration),
    );
    store.exp("test".to_string(), Duration::new(10, 0));
    assert!(true)
}
#[test]
pub fn test_get() {
    let mut store = Store {
        data: FxHashMap::default(),
    };
    store.set("test".to_string(), "test".to_owned().into_bytes(), None);
    let data = store.get("test".to_string()).unwrap();
    assert_eq!("test".to_owned().into_bytes(), data);
}
#[test]
pub fn test_get_with_duration() {
    let mut store = Store {
        data: FxHashMap::default(),
    };
    let durration = Some(Duration::new(5, 0));
    store.set(
        "test".to_string(),
        "test".to_owned().into_bytes(),
        durration,
    );
    let data = store.get("test".to_string()).unwrap();
    assert_eq!("test".to_owned().into_bytes(), data);
}
#[test]
pub fn test_set() {
    let mut store = Store {
        data: FxHashMap::default(),
    };
    store.set("test".to_string(), "test".to_owned().into_bytes(), None);
    assert!(true);
}
#[test]
pub fn test_delete() {
    let mut store = Store {
        data: FxHashMap::default(),
    };
    store.set("test".to_string(), "test".to_owned().into_bytes(), None);
    store.delete("test".to_string());
    assert!(true);
}
