use crate::soldier::Soldier;
use crate::rank::Rank;
use regex::Regex;
use slotmap::{SlotMap, DefaultKey};
use serde::{Serialize, Deserialize};

type SoldierKey = DefaultKey;
#[derive(Serialize, Deserialize)]
pub struct SoldierExtent {
    pub all : SlotMap<SoldierKey, Soldier>
}

impl SoldierExtent {
    pub fn new() -> SoldierExtent {
        SoldierExtent {
            all : SlotMap::new()
        }
    }

    // create a new soldier and return their key
    pub fn create(&mut self, name : String, rank : Rank) -> SoldierKey {
        let soldier = Soldier {
            name : name,
            rank : rank
        };
        self.all.insert(soldier)
    }

    // search in all soldiers and return a ref to the first one with the given name
    pub fn key_by_name(&self, name : &str) -> Option<DefaultKey> {
        for (key, soldier) in self.all.iter() {
            if soldier.name == name {
                return Some(key);
            }
        }
        None
    }

    // search in all soldiers and return a ref to all matching the regex in their name
    pub fn key_by_name_regex(&self, regex: &Regex) -> Vec<DefaultKey> {
        let mut result = Vec::new();
        for (key, soldier) in self.all.iter() {
            if regex.is_match(&soldier.name) {
                result.push(key);
            }
        }
        result
    }

    // return a valid ref to all soldiers having a certain rank
    pub fn key_by_rank(&self, rank : Rank) -> Vec<DefaultKey> {
        let mut result = Vec::new();
        for (key, soldier) in self.all.iter() {
            if soldier.rank == rank {
                result.push(key);
            }
        }
        result
    }

    // return a valid ref to soldier if it exists
    pub fn get(&self, key : SoldierKey) -> Option<&Soldier> {
        self.all.get(key)
    }

    pub fn get_multiple(&self, keys : Vec<SoldierKey>) -> Vec<&Soldier> {
        let mut result = Vec::new();
        for key in keys {
            if let Some(soldier) = self.all.get(key) {
                result.push(soldier);
            }
        }
        result
    }

    pub fn get_mut(&mut self, key : SoldierKey) -> Option<&mut Soldier> {
        self.all.get_mut(key)
    }

    // remove a soldier from the extent
    pub fn remove(&mut self, key : SoldierKey) {
        self.all.remove(key);
    }

    pub fn save_all(&self, path : &str) {
        let serialized = serde_json::to_string(&self.all).unwrap();
        std::fs::write(path, serialized).unwrap();
    }

    pub fn load_from_file(&mut self, path : &str) {
        let serialized = std::fs::read_to_string(path).unwrap();
        let deserialized : SlotMap<SoldierKey, Soldier> = serde_json::from_str(&serialized).unwrap();
        self.all = deserialized;
    }

}

#[cfg(test)]
mod tests {
    use crate::{soldier_extent::SoldierExtent, rank::Rank};

    #[test]
    fn create() {
        let mut soldiers = SoldierExtent::new();
        soldiers.create("John".to_string(), Rank::Private);
        soldiers.create("John2".to_string(), Rank::Private);
        assert!(soldiers.key_by_name("John").is_some());
        assert!(soldiers.key_by_name("John2").is_some());
        assert!(soldiers.key_by_rank(Rank::Private).len() == 2);
    }

    #[test]
    fn save() {
        let mut soldiers = SoldierExtent::new();
        soldiers.create("John".to_string(), Rank::Private);
        soldiers.create("John2".to_string(), Rank::Private);
        let path = "soldiers.json";
        soldiers.save_all(path);
        assert!(std::fs::read(path).is_ok());
    }

    #[test]
    fn save_and_load() {
        let mut soldiers = SoldierExtent::new();
        soldiers.create("John".to_string(), Rank::Private);
        soldiers.create("John2".to_string(), Rank::Private);
        let path = "soldiers.json";
        soldiers.save_all(path);
        let mut soldiers2 = SoldierExtent::new();
        soldiers2.load_from_file(path);
        assert!(soldiers2.key_by_name("John").is_some());
        assert!(soldiers2.key_by_name("John2").is_some());
        assert!(soldiers2.key_by_rank(Rank::Private).len() == 2);
    }

    #[test]
    fn mutate() {
        let mut soldiers = SoldierExtent::new();
        let key = soldiers.create("John".to_string(), Rank::Private);
        { // mutate in a separate scope to keep ownership happy
            let soldier = soldiers.get_mut(key);
            assert!(soldier.is_some()); // test if we can retrieve them
            let soldier = soldier.unwrap(); // a mutable ref to the soldier
            assert!(soldier.name == "John"); // test if the name is correct
            assert!(soldier.rank == Rank::Private); // test if the rank is correct
            soldier.name = "Bob".to_string(); // mutate the name
            soldier.rank = Rank::Captain; // mutate the rank
            // ref is dropped by the end of this scope
        }

        { // always do ops on the retrieved refs in closed scopes
            let soldier = soldiers.get(key);
            assert!(soldier.is_some()); // test if we can retrieve them
            let soldier = soldier.unwrap(); // a ref to the soldier
            assert!(soldier.name == "Bob"); // test if the name is correct
            assert!(soldier.rank == Rank::Captain); // test if the rank is correct
            // ref is dropped by the end of this scope
        }
    }

    #[test]
    fn find_with_regex() {
        use regex::Regex;

        let mut soldiers = SoldierExtent::new();
        soldiers.create("John".to_string(), Rank::Private);
        soldiers.create("Joe".to_string(), Rank::Private);
        soldiers.create("Bob".to_string(), Rank::Private);

        let reg = Regex::new(r"Jo.*").unwrap();
        let filtered_soldiers = soldiers.key_by_name_regex(&reg);
        assert!(filtered_soldiers.len() == 2);
    }

    #[test]
    fn remove() {
        let mut soldiers = SoldierExtent::new();
        let key = soldiers.create("John".to_string(), Rank::Private);
        soldiers.create("Joe".to_string(), Rank::Private);

        soldiers.remove(key);

        assert!(soldiers.key_by_name("John").is_none());
    }
}