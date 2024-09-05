use serde_json::Result;

use std::fs;

use slint::{ModelRc,Model};
use crate::mvc;

#[derive(Clone)]
pub struct MenuController{
    file_location:String
}

impl MenuController{
    pub fn new() -> Self{
        Self {file_location: "Uhhh".into()}
    }
    pub fn save_ship_list(&self, ships:Vec<mvc::ShipModel>){
        let mut contents:String = String::new();
        //This format doesn't relaly work, need to use it over an entire vector
        for ship in ships{
            contents += &serde_json::to_string_pretty(&ship).unwrap();
        }
        fs::write("D:/rust/swn_helper/output/text.json", contents).unwrap();
    }

    pub fn load_ship_list(){

    }

    
}
