use std::error::Error;

pub trait Table<K, V> {
    fn put(&self, key: K, value: V) -> Result<(), Box<dyn Error>>;
    fn get(&self, key: K) -> Result<Option<V>, Box<dyn Error>>;
    fn delete(&self, key: K) -> Result<(), Box<dyn Error>>;
}

trait TableSerialize<K, V> {
    fn serialize_key(&self, key: &K) -> Result<Vec<u8>, Box<dyn Error>>;
    fn deserialize_key(&self, bytes: &[u8]) -> Result<K, Box<dyn Error>>;
    fn serialize_value(&self, value: &V) -> Result<Vec<u8>, Box<dyn Error>>;
    fn deserialize_value(&self, bytes: &[u8]) -> Result<V, Box<dyn Error>>;
}

struct StringStringTable {}

impl Table<String, String> for StringStringTable {
    fn put(&self, key: String, value: String) -> Result<(), Box<dyn Error>> {
        // Implementation for putting a key-value pair
        Ok(())
    }

    fn get(&self, key: String) -> Result<Option<String>, Box<dyn Error>> {
        // Implementation for getting a value by key
        Ok(None)
    }

    fn delete(&self, key: String) -> Result<(), Box<dyn Error>> {
        // Implementation for deleting a key-value pair
        Ok(())
    }
}

impl TableSerialize<String, String> for StringStringTable {
    fn serialize_key(&self, key: &String) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(key.as_bytes().to_vec())
    }

    fn deserialize_key(&self, bytes: &[u8]) -> Result<String, Box<dyn Error>> {
        String::from_utf8(bytes.to_vec()).map_err(|e| e.into())
    }

    fn serialize_value(&self, value: &String) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(value.as_bytes().to_vec())
    }

    fn deserialize_value(&self, bytes: &[u8]) -> Result<String, Box<dyn Error>> {
        String::from_utf8(bytes.to_vec()).map_err(|e| e.into())
    }
}
