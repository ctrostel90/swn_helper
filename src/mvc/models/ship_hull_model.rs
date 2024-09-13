use serde::{Serialize,Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

impl Default for ShipHullModel{
    fn default() -> Self {
        Self{
            name:"".into(),
            cost:0,
            speed:0,
            armor:0,
            hp:0,
            crew_minimum:0,
            crew_maximum:0,
            ac:0,
            power:0,
            mass:0,
            hard_points:0,
            class:"Fighter".into(),
        }
    }
}