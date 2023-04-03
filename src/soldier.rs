use crate::rank::Rank;
use serde::{Serialize, Deserialize};

pub trait Pay {
    // metoda do nadpisania w klasach pochodnych
    fn calculate_pay(&self) -> f64;
}

#[typetag::serde(tag = "type")]
pub trait Soldier: Pay {

    fn get_name(&self) -> &str;
    fn set_name(&mut self, new_name: String);

    fn get_rank(&self) -> &Rank;
    fn set_rank(&mut self, new_rank: Rank);

    fn get_family_address(&self) -> Option<&String> {
        None
    }

    fn new(name: String, rank: Option<Rank>) -> Self where Self: Sized;
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct StandardSoldier {
    pub name : String,
    pub rank : Rank,
    pub family_address: Option<String>, // atrybut opcjonalny
}

impl StandardSoldier {
    pub fn change_family_address(&mut self, new_address: String) {
        self.family_address = Some(new_address);
    }

    pub fn remove_family_address(&mut self) {
        self.family_address = None;
    }

    pub fn promote(&mut self) {
        self.rank = self.rank.promote();
    }

    pub fn demote(&mut self) {
        self.rank = self.rank.demote();
    }

    // atrybut "klasowy"
    pub const DEFAULT_RANK: Rank = Rank::Private;
}

#[typetag::serde]
impl Soldier for StandardSoldier {

    fn set_name(&mut self, new_name: String) { self.name = new_name }
    fn get_name(&self) -> &str { &self.name }
    fn set_rank(&mut self, new_rank: Rank) { self.rank = new_rank }
    fn get_rank(&self) -> &Rank { &self.rank }

    // metoda klasowa
    fn new(name: String, rank: Option<Rank>) -> StandardSoldier {

        let rank = match rank {
            Some(r) => r,
            None => StandardSoldier::DEFAULT_RANK,
        };

        StandardSoldier {
            name,
            rank,
            family_address: None,
        }
    }
}

impl Pay for StandardSoldier {
    fn calculate_pay(&self) -> f64 {
        match self.rank {
            Rank::Private => 1000.0,
            Rank::Corporal => 2000.0,
            Rank::Sergeant => 3000.0,
            Rank::Lieutenant => 4000.0,
            Rank::Captain => 5000.0,
            Rank::Major => 6000.0,
            Rank::Colonel => 7000.0,
            Rank::General => 8000.0,
        }
    }
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Engineer {
    // bycmoze mozna inaczej zapisac pola klasy bazowej
    pub name : String,
    pub rank : Rank,
    pub family_address: Option<String>, // atrybut opcjonalny
    pub specialization: Option<String>, // x2
}

impl Engineer {
    pub fn change_specialization(&mut self, new_specialization: String) {
        self.specialization = Some(new_specialization);
    }
}

impl Pay for Engineer {
    fn calculate_pay(&self) -> f64 {
        3500.0 // engineers have a flat pay
    }
}

#[typetag::serde]
impl Soldier for Engineer {

    // you gotta do what you gotta do
    // copy pasting is performance friendly >:D
    fn set_name(&mut self, new_name: String) { self.name = new_name }
    fn get_name(&self) -> &str { &self.name }
    fn set_rank(&mut self, new_rank: Rank) { self.rank = new_rank }
    fn get_rank(&self) -> &Rank { &self.rank }

    // metoda klasowa xD
    fn new(name: String, rank: Option<Rank>) -> Engineer {
        let rank = match rank {
            Some(r) => r,
            None => Rank::Corporal, // all engineers start as corporals
        };

        Engineer {
            name,
            rank,
            family_address: None,
            specialization: None
        }
    }
}