

#[derive(Debug)]
pub struct KvStore {}

impl KvStore {
    pub fn new() -> Self { 
        KvStore {

        }
    }

    pub fn set(&mut self, key: String, value: String) {
        panic!();
    }

    pub fn get(&mut self, key: String) -> Option<String> {
        Some(String::from("algo"))
    }

    pub fn remove(&mut self, key: String) {
        panic!();
    }
}