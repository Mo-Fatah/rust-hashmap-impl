use std::{hash::{Hash, Hasher}, collections::hash_map::DefaultHasher, mem};

const INITIAL_N_BUCKETS: usize = 1;

pub struct HashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    items: usize,
}

impl<K, V> HashMap<K, V> {
    pub fn new() -> Self {
        HashMap {
            buckets: Vec::new(),
            items: 0,
        }
    }
}

impl<K, V> HashMap<K, V>
where
    K: Hash + Eq, { 

    fn bucket_index(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() % self.buckets.len() as u64) as usize
    }
    
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.buckets.is_empty() || self.items > 3 * self.buckets.len() / 4 {
            self.resize();
        }        

        let bucket_index = self.bucket_index(&key);  
        let bucket = &mut self.buckets[bucket_index];

        self.items += 1;
        for &mut (ref ekey, ref mut  evalue) in bucket.iter_mut() {
            if *ekey == key {
                let old_value = mem::replace(evalue, value);
                return Some(old_value);
            }
        }

        bucket.push((key, value));
        None
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.buckets[self.bucket_index(key)]
            .iter()
            .find(|(ekey, _)| ekey == key)
            .map(|(_, ref v)| v)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let bucket_index = self.bucket_index(&key);
        let bucket = &mut self.buckets[bucket_index];
        let i = bucket.iter().position(|&(ref ekey, _)|  ekey == key)?;
        let removed_value = bucket.swap_remove(i);
        self.items -= 1;
        Some(removed_value.1)
        
    }


    fn resize(&mut self) {
        let target_size = match self.buckets.len() {
            0 => INITIAL_N_BUCKETS,
            n => 2 * n,
        };
        let mut new_buckets: Vec<Vec<(K, V)>> = Vec::with_capacity(target_size);
        new_buckets.extend((0..target_size).map(|_| Vec::new()));

        for (key, value) in self.buckets.iter_mut().flat_map(|bucket| bucket.drain(..)) {
            let mut hasher = DefaultHasher::new();
            key.hash(&mut hasher);
            let bucket_index: usize = (hasher.finish() % new_buckets.len() as u64) as usize;
            let bucket = &mut new_buckets[bucket_index];
            bucket.push((key, value));
        }
        mem::replace(&mut self.buckets, new_buckets);
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn insert() {
        let mut map = HashMap::new();
        map.insert("key", "value");
        assert_eq!(map.get(&"key"), Some(&"value"));
        assert_eq!(map.remove(&"key"), Some("value"));
        assert_eq!(map.get(&"key"), None);

    }
}



        



