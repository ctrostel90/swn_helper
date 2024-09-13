//Handle all of the loading of libraries at startups
use crate::mvc::{ShipFittingsModel, ShipHullModel, ShipWeaponModel};
use serde_json::Result;
use std::{fs, rc::Rc};

pub struct AppConfig{
    ship_fittings_import_file_path:String,
    ship_weapons_import_file_path:String,
    ship_hulls_import_file_path:String,
}

impl AppConfig{
    pub fn new(
        ship_fittings_path:String,
        ship_weapons_path:String,
        ship_hulls_path:String,
    ) -> Self{
        Self{
            ship_fittings_import_file_path: ship_fittings_path,
            ship_weapons_import_file_path: ship_weapons_path,
            ship_hulls_import_file_path: ship_hulls_path,
        }
    }

    pub fn startup(&self) -> (Vec<ShipHullModel>,Vec<ShipWeaponModel>, Vec<ShipFittingsModel>){
        //todo: Error handling here is non existant
        let mut ship_hull_list:Vec<ShipHullModel> = vec![ShipHullModel::default()];
        let mut ship_weapon_list:Vec<ShipWeaponModel>= vec![ShipWeaponModel::default()];;
        let mut ship_fittings_list:Vec<ShipFittingsModel>= vec![ShipFittingsModel::default()];;
        match fs::read_to_string(&self.ship_hulls_import_file_path) {
            Err(_) => {},
            Ok(contents) => {
                ship_hull_list = serde_json::from_str(&contents).unwrap();
            }
        }
        match fs::read_to_string(&self.ship_weapons_import_file_path) {
            Err(_) => {},
            Ok(contents) => {
                ship_weapon_list = serde_json::from_str(&contents).unwrap();
            }
        }
        match fs::read_to_string(&self.ship_fittings_import_file_path) {
            Err(_) => {},
            Ok(contents) => {
                ship_fittings_list = serde_json::from_str(&contents).unwrap();
            }
        }
            
        (ship_hull_list,ship_weapon_list, ship_fittings_list)
    }
}