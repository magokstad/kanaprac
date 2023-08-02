use std::io;
use std::collections::{HashMap, HashSet};
use rand::seq::IteratorRandom;

pub struct Mapping {
    map: HashMap<String, Vec<String>>,
    kana: HashSet<String>,
    work_set: HashSet<String>,
    loops: i32
}

impl Mapping {
    fn new(map: &HashMap<String, Vec<String>>, kana: &HashSet<String>) -> Self {
        Self { 
            map: map.clone(), 
            kana: kana.clone(), 
            work_set: kana.clone(), 
            loops: 0 
        }
    }

    pub fn from(data: &str) -> io::Result<Self> {
        Mapping::parse_mapping(data)
    }

    pub fn join(&self, other: &Mapping) -> Mapping {
        let map = self.map
            .clone()
            .into_iter()
            .chain(other.map.clone().into_iter())
            .collect();
        let kana = self.kana
            .clone()
            .into_iter()
            .chain(other.kana.clone().into_iter())
            .collect();
        return Mapping::new(&map, &kana);
    }

    pub fn get_random(&mut self) -> String {
        self.ensure_not_empty();
        self.work_set
            .iter()
            .choose(&mut rand::thread_rng())
            .expect("bad").to_owned()
    }

    pub fn remove(&mut self, kana: &String) -> bool {
        self.work_set.remove(kana);
        self.ensure_not_empty()
    }

    pub fn get_romaji_from(&self, kana: &str) -> Vec<String> {
        self.map.get(kana).unwrap().clone()
    }

    pub fn work_set_status(&self) -> (usize, usize) {
        (self.work_set.len(), self.kana.len())
    }

    fn ensure_not_empty(&mut self) -> bool {
        if self.work_set.is_empty() {
            self.work_set = self.kana.clone();
            self.loops += 1;
            return false;
        } 
        return true;
    }

    fn parse_mapping(data: &str) -> io::Result<Mapping> {
        let mut map : HashMap<String, Vec<String>> = HashMap::new(); 
        let mut kana: HashSet<String> = HashSet::new();

        for line in data.lines() {
            let parts: Vec<&str> = line.split_ascii_whitespace().collect();
            if parts.len() == 0 {
                continue
            }
            if parts.len() == 1 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData, 
                    format!("Bad number of arguments on this line: {}", line)
                ));
            }
            map.entry(parts[0].to_string()).or_insert(vec![]);
            for i in 1..parts.len() {
                map.get_mut(&parts[0].to_string())
                    .unwrap()
                    .push(parts[i].to_string());
            }
            kana.insert(parts[0].to_string());
        }

        Ok(Mapping::new(&map, &kana))
    }
}