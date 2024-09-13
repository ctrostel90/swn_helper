use crate::mvc::ShipHullModel;
use serde::{Deserialize, Serialize};

use super::ShipWeaponModel;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShipModel {
    pub name: String,
    pub hull: ShipHullModel,
    pub hp: i64,
    pub npc_cp_count: i64,
    pub weapon_fittings: Vec<ShipWeaponModel>,
    pub fitting_one: String,
}

impl ShipModel {
    pub fn new(
        ship_name: String,
        ship_hull: ShipHullModel,
        ship_hp: i64,
        ship_npc_cp_count: i64,
        ship_weapon_fittings: Vec<ShipWeaponModel>,
        ship_fitting_one: String,
    ) -> Self {
        Self {
            name: ship_name,
            hull: ship_hull,
            hp: ship_hp,
            npc_cp_count: ship_npc_cp_count,
            weapon_fittings: ship_weapon_fittings,
            fitting_one: ship_fitting_one,
        }
    }
    pub fn get_cost(&self) -> i64 {
        self.hull.cost
    }
    pub fn get_crew(&self) -> String {
        format!("{}/{}", self.get_crew_minimum(), self.get_crew_maximum())
    }
    pub fn get_crew_minimum(&self) -> i64 {
        self.hull.crew_minimum
    }
    pub fn get_crew_maximum(&self) -> i64 {
        self.hull.crew_maximum
    }
    pub fn get_number_hard_points(&self) -> i64 {
        self.hull.hard_points
    }
    pub fn get_remaining_hard_points(&self) -> i64{
        self.hull.hard_points - (self.weapon_fittings.iter().map(|x| x.hard_points).sum::<i64>())
    }
    pub fn get_armor(&self) -> i64 {
        self.hull.armor
    }
    pub fn get_ac(&self) -> i64 {
        self.hull.ac
    }
    pub fn get_power(&self) -> i64 {
        self.hull.power
    }
    pub fn get_mass(&self) -> i64 {
        self.hull.mass
    }
    pub fn get_speed(&self) -> i64 {
        self.hull.speed
    }
}
