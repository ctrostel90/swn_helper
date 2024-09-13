use serde::{Deserialize, Serialize};
use slint::Model;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShipWeaponModel {
    pub name: String,
    pub cost: i64,
    pub damage: String,
    pub power: i64,
    pub mass: i64,
    pub hard_points: i64,
    pub class: String,
    pub tech_level: i64,
    pub qualities: String,
}

impl Default for ShipWeaponModel {
    fn default() -> Self {
        Self {
            name: "".into(),
            cost: 0,
            damage: "".into(),
            power: 0,
            mass: 0,
            hard_points: 0,
            class: "Fighter".into(),
            tech_level: 0,
            qualities: "".into(),
        }
    }
}
