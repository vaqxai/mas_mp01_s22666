use crate::soldier::{Soldier, StandardSoldier};
use crate::rank::Rank;
use regex::Regex;
use slotmap::{SlotMap, DefaultKey};
use serde::{Serialize, Deserialize};

type SoldierKey = DefaultKey;

#[derive(Serialize, Deserialize)]
pub struct SoldierExtent {
    pub all : SlotMap<SoldierKey, Box<dyn Soldier>>
}

type DefaultSoldierType = StandardSoldier;

// "Prawdziwy" overloading
macro_rules! create_soldier{

    (@gobble $str:expr) => {{
        trait StrGobbler {
            fn gobble(self) -> String;
        }
    
        impl StrGobbler for &str {
            fn gobble(self) -> String { self.to_string() }
        }
    
        impl StrGobbler for String {
            fn gobble(self) -> String { self }
        }

        $str.gobble()
    }};

    ($sldext:expr, $name:expr) => {
        create_soldier!($sldext, $name, DefaultSoldierType)
    };

    ($sldext:expr, $name:expr, $typ:ty) => {
        $sldext.create::<$typ>(create_soldier!(@gobble $name))
    };

    ($sldext:expr, $name:expr, $rank:expr) => {
        create_soldier!($sldext, create_soldier!(@gobble $name), $rank, DefaultSoldierType)
    };

    ($sldext:expr, $name:expr, $rank:expr, $typ:ty) => {
        $sldext.create_with_rank::<$typ>(create_soldier!(@gobble $name), $rank)
    };

}

impl SoldierExtent {
    pub fn new() -> SoldierExtent {
        SoldierExtent {
            all : SlotMap::new()
        }
    }

    // Można to uznać za przeciążenie moim zdaniem
    // "prawdziwe" przeciążenie (gdzie nazwa funkcji jest zawsze taka sama) jest możliwe tylko przy użyciu makr
    // dlatego mamy zrobione makro create_soldier!(extent, name) lub create_soldier!(extent, name, rank)
    
    // default rank
    pub fn create<T: Soldier + 'static>(&mut self, name: String) -> SoldierKey {
        self.create_with_rank::<T>(name, None)
    }

    // create a new soldier and return their key
    pub fn create_with_rank<T: Soldier + 'static>(&mut self, name : String, rank : Option<Rank>) -> SoldierKey {
        let soldier = T::new(name, rank);
        self.all.insert(Box::new(soldier))
    }

    // search in all soldiers and return a ref to the first one with the given name
    pub fn key_by_name(&self, name : &str) -> Option<DefaultKey> {
        for (key, soldier) in self.all.iter() {
            if soldier.get_name() == name {
                return Some(key);
            }
        }
        None
    }

    // search in all soldiers and return a ref to all matching the regex in their name
    pub fn key_by_name_regex(&self, regex: &Regex) -> Vec<DefaultKey> {
        let mut result = Vec::new();
        for (key, soldier) in self.all.iter() {
            if regex.is_match(&soldier.get_name()) {
                result.push(key);
            }
        }
        result
    }

    // return a valid ref to all soldiers having a certain rank
    pub fn key_by_rank(&self, rank : Rank) -> Vec<DefaultKey> {
        let mut result = Vec::new();
        for (key, soldier) in self.all.iter() {
            if soldier.get_rank() == &rank {
                result.push(key);
            }
        }
        result
    }

    // return a valid ref to soldier if it exists
    pub fn get(&self, key : SoldierKey) -> Option<&dyn Soldier> {
        match self.all.get(key) {
            Some(soldier) => Some(soldier.as_ref()),
            None => None
        }
    }

    pub fn get_multiple(&self, keys : Vec<SoldierKey>) -> Vec<&dyn Soldier> {
        let mut result = Vec::new();
        for key in keys {
            if let Some(soldier) = self.all.get(key) {
                result.push(soldier.as_ref());
            }
        }
        result
    }

    pub fn get_mut(&mut self, key : SoldierKey) -> Option<&mut dyn Soldier> {
        match self.all.get_mut(key) {
            Some(soldier) => Some(soldier.as_mut()),
            None => None
        }
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
        let deserialized : SlotMap<SoldierKey, Box<dyn Soldier>> = serde_json::from_str(&serialized).unwrap();
        self.all = deserialized;
    }

}

#[cfg(test)]
mod tests {
    use crate::{soldier_extent::{SoldierExtent, DefaultSoldierType}, rank::Rank};

    #[test]
    fn create() {
        let mut soldiers = SoldierExtent::new();
        create_soldier!(soldiers, "John");
        create_soldier!(soldiers, "John2");
        assert!(soldiers.key_by_name("John").is_some());
        assert!(soldiers.key_by_name("John2").is_some());
        assert!(soldiers.key_by_rank(DefaultSoldierType::DEFAULT_RANK).len() == 2);
    }

    #[test]
    fn save() {
        let mut soldiers = SoldierExtent::new();
        create_soldier!(soldiers, "John");
        create_soldier!(soldiers, "John2");
        let path = "soldiers.json";
        soldiers.save_all(path);
        assert!(std::fs::read(path).is_ok());
    }

    #[test]
    fn save_and_load() {
        let mut soldiers = SoldierExtent::new();
        create_soldier!(soldiers, "John");
        create_soldier!(soldiers, "John2");
        let path = "soldiers.json";
        soldiers.save_all(path);
        let mut soldiers2 = SoldierExtent::new();
        soldiers2.load_from_file(path);
        assert!(soldiers2.key_by_name("John").is_some());
        assert!(soldiers2.key_by_name("John2").is_some());
        assert!(soldiers2.key_by_rank(DefaultSoldierType::DEFAULT_RANK).len() == 2);
    }

    #[test]
    fn mutate() {
        let mut soldiers = SoldierExtent::new();
        let key = create_soldier!(soldiers, "John");
        { // mutate in a separate scope to keep ownership happy
            let soldier = soldiers.get_mut(key);
            assert!(soldier.is_some()); // test if we can retrieve them
            let soldier = soldier.unwrap(); // a mutable ref to the soldier
            assert!(soldier.get_name() == "John"); // test if the name is correct
            assert!(soldier.get_rank() == &DefaultSoldierType::DEFAULT_RANK); // test if the rank is correct
            soldier.set_name("Bob".to_string()); // mutate the name
            soldier.set_rank(Rank::Captain); // mutate the rank
            // ref is dropped by the end of this scope
        }

        { // always do ops on the retrieved refs in closed scopes
            let soldier = soldiers.get(key);
            assert!(soldier.is_some()); // test if we can retrieve them
            let soldier = soldier.unwrap(); // a ref to the soldier
            assert!(soldier.get_name() == "Bob"); // test if the name is correct
            assert!(soldier.get_rank() == &Rank::Captain); // test if the rank is correct
            // ref is dropped by the end of this scope
        }
    }

    #[test]
    fn find_with_regex() {
        use regex::Regex;

        let mut soldiers = SoldierExtent::new();
        create_soldier!(soldiers, "John");
        create_soldier!(soldiers, "Joe");
        create_soldier!(soldiers, "Bob");

        let reg = Regex::new(r"Jo.*").unwrap();
        let filtered_soldiers = soldiers.key_by_name_regex(&reg);
        assert!(filtered_soldiers.len() == 2);
    }

    #[test]
    fn remove() {
        let mut soldiers = SoldierExtent::new();
        let key = create_soldier!(soldiers, "John");
        create_soldier!(soldiers, "Joe");

        soldiers.remove(key);

        assert!(soldiers.key_by_name("John").is_none());
    }
}