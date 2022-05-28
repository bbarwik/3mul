use intmap::IntMap;
use std::option::Option;

const VEC_SIZE : usize = 65536;

pub struct FastHashSet {
    vec: Vec<Vec<usize>>,
    map: IntMap<Vec<usize>>
}

impl FastHashSet {
    pub fn new() -> FastHashSet {
        let mut vec = Vec::new();
        vec.resize(VEC_SIZE, Vec::new());
        FastHashSet{
            vec,
            map: IntMap::new()
        }
    }

    pub fn insert(&mut self, key: u64, value: Vec<usize>) {
        if key >= VEC_SIZE as u64 {
            self.map.insert(key, value);
        }
    }

    pub fn get(&self, key: u64) -> Option<&Vec<usize>> {
        if key >= VEC_SIZE as u64 {
            return self.map.get(key);
        }
        let vec = &self.vec[key as usize];
        if vec.len() == 0 {
            None
        } else {
            Some(vec)
        }
    }

    pub fn get_mut(&mut self, key: u64) -> Option<&mut Vec<usize>> {
        if key >= VEC_SIZE as u64 {
            return self.map.get_mut(key);
        }
        Some(&mut self.vec[key as usize])
    }

    pub fn remove(&mut self, key: u64) {
        if key >= VEC_SIZE as u64 {
            self.map.remove(key);
        } else {
            self.get_mut(key).unwrap().clear();
        }
    }
}
