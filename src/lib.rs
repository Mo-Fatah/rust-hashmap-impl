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

        for &mut (ref ekey, ref mut evalue) in bucket.iter_mut() {
            if *ekey == key {
                let old_value = mem::replace(evalue, value);
                return Some(old_value);
            }
        }
        self.items += 1;
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

    pub fn len(&self) -> usize {
        self.items
    }

    pub fn is_empty(&self) -> bool {
        self.items == 0
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.get(&key).is_some()
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

pub struct Iter<'a, K: 'a, V: 'a> {
    map: &'a HashMap<K, V>,
    bucket: usize,
    at: usize,
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.map.buckets.get(self.bucket){
                Some(bucket) => {
                    match bucket.get(self.at) {

                        Some(&(ref k, ref v)) => {
                            self.at += 1;
                            break Some((k, v));
                        }
                        None => {
                            self.bucket += 1;
                            continue;
                        }
                    } 
                } 

                None => {
                    break None; 
                }
            }
        } 
    }
}

impl<'a, K, V> IntoIterator for &'a HashMap<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;
    fn into_iter(self) -> Self::IntoIter {
        Iter {
            map: self,
            bucket: 0,
            at: 0
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn insert() {
        let mut map = HashMap::new();
        assert_eq!(map.len(), 0);
        assert_eq!(map.is_empty(), true);
        map.insert("key", "value");
        assert_eq!(map.len(), 1);
        assert_eq!(map.get(&"key"), Some(&"value"));
        map.insert("key", "value2");
        assert!(map.contains_key(&"key"));
        assert_eq!(map.get(&"key"), Some(&"value2"));
        assert_eq!(map.remove(&"key"), Some("value2"));
        assert_eq!(map.get(&"key"), None);
        assert_eq!(map.len(), 0);
        assert_eq!(map.is_empty(), true);
    }

    #[test]
    fn iter_test() {
        let mut map = HashMap::new();
        map.insert("foo", 1);
        map.insert("boo", 2);
        map.insert("zoo", 3);
        map.insert("doo", 4);
        for (k, v) in &map {
            match *k {
                "foo" => assert_eq!(*v, 1),
                "boo" => assert_eq!(*v, 2),
                "zoo" => assert_eq!(*v, 3),
                "doo" => assert_eq!(*v, 4),
                _ => unimplemented!(),
            }
        }
    }
}



        



