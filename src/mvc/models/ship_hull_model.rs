use serde::{Serialize,Deserialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShipHullModel {
    pub name: String,
    pub cost: i64,
    pub speed: i64,
    pub armor: i64,
    pub hp:i64,
    pub crew_minimum: i64,
    pub crew_maximum: i64,
    pub ac: i64,
    pub power: i64,
    pub mass: i64,
    pub hard_points: i64,
    pub class: String,
}

impl ShipHullModel{
    pub fn get_crew(&self) -> String {
        format!("{}/{}", self.crew_minimum, self.crew_maximum)
    }
}
