use serde::{Serialize,Deserialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShipModel {
    pub name: String,
    pub hull: String,
    pub class: String,
    pub hp: i64,
    pub crew_minimum: i64,
    pub crew_maximum: i64,
    pub armor: i64,
    pub ac: i64,
    pub power: i64,
    pub mass: i64,
    pub npc_cp_count: i64,
    pub speed: i64,
    pub weapon_one: String,
    pub fitting_one: String,
}
