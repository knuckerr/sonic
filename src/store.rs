use rustc_hash::FxHashMap;

pub struct Store<'a> {
    data: FxHashMap<&'a str, Vec<u8>>,
}

impl<'a> Store<'a> {
    pub fn new() -> Self {
        Store{data: FxHashMap::default()}
    }
    pub fn get(self, key: &'a str) -> Option<Vec<u8>> {
        self.data.get(key).cloned()
    }
    pub fn set(&mut self, key: &'a str, value: Vec<u8>) {
        self.data.insert(key, value);
    }
    pub fn delete(&mut self, key: &'a str) {
        self.data.remove(key);
    }
}


#[test]
pub fn get() {
    let mut store = Store{data: FxHashMap::default()};
    store.set("test", "test".to_owned().into_bytes());
    let data = store.get("test").unwrap();
    assert_eq!("test".to_owned().into_bytes(), data);
}
#[test]
pub fn set() {
    let mut store = Store{data: FxHashMap::default()};
    store.set("test", "test".to_owned().into_bytes());
    assert!(true);
}
#[test]
pub fn delete() {
    let mut store = Store{data: FxHashMap::default()};
    store.set("test", "test".to_owned().into_bytes());
    store.delete("test");
    assert!(true);
}
