use crate::rank::Rank;
use serde::{Serialize, Deserialize};

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Soldier {
    pub name : String,
    pub rank : Rank
}