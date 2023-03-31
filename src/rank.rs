use serde::{Serialize, Deserialize};

#[derive(PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Rank {
    Private, Corporal, Sergeant, Lieutenant, Captain, Major, Colonel, General
}

// TODO: Are we sure we want to declare all of this manually?

impl Rank {
    pub fn from_string(s : &str) -> Option<Rank> {
        match s {
            "Private" => Some(Rank::Private),
            "Corporal" => Some(Rank::Corporal),
            "Sergeant" => Some(Rank::Sergeant),
            "Lieutenant" => Some(Rank::Lieutenant),
            "Captain" => Some(Rank::Captain),
            "Major" => Some(Rank::Major),
            "Colonel" => Some(Rank::Colonel),
            "General" => Some(Rank::General),
            _ => None
        }
    }

    pub fn promote(self) -> Rank {
        match self {
            Rank::Private => Rank::Corporal,
            Rank::Corporal => Rank::Sergeant,
            Rank::Sergeant => Rank::Lieutenant,
            Rank::Lieutenant => Rank::Captain,
            Rank::Captain => Rank::Major,
            Rank::Major => Rank::Colonel,
            Rank::Colonel => Rank::General,
            Rank::General => Rank::General
        }
    }

    pub fn demote(self) -> Rank {
        match self {
            Rank::Private => Rank::Private,
            Rank::Corporal => Rank::Private,
            Rank::Sergeant => Rank::Corporal,
            Rank::Lieutenant => Rank::Sergeant,
            Rank::Captain => Rank::Lieutenant,
            Rank::Major => Rank::Captain,
            Rank::Colonel => Rank::Major,
            Rank::General => Rank::Colonel
        }
    }

    pub fn reset(self) -> Rank {
        Rank::Private
    }

}