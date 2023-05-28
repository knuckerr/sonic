use rustc_hash::FxHashMap;

struct Store<'a> {
    data: FxHashMap<&'a str, Vec<u8>>,
}

impl<'a> Store<'a> {
    pub fn get(self, key: &'a str) -> Option<&[u8]> {
        self.data.get(key).map(|value| &value[..])
    }
    pub fn set(&mut self, key: &'a str, value: Vec<u8>) {
        self.data.insert(key, value);
    }
    pub fn delete(&mut self, key: &'a str) {
        self.data.remove(key);
    }
}
