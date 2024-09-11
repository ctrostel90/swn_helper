use serde_json::Result;

use std::{fs, rc::Rc};

use slint::{ModelRc,Model};
use crate::{mvc::{self, ShipModel,ShipHullModel}, Callback};

use rfd::FileDialog;

#[derive(Clone)]
pub struct MenuController{
    file_location:String,
    load_ship_list_callback: Rc<Callback<Vec<mvc::ShipModel>,()>>,
    load_ship_hull_list_callback: Rc<Callback<Vec<mvc::ShipHullModel>,()>>,
}

impl MenuController{
    pub fn new() -> Self{
        Self {
            file_location: "Uhhh".into(),
            load_ship_list_callback: Rc::new(Callback::default()),
            load_ship_hull_list_callback: Rc::new(Callback::default()),
        }
    }
    pub fn save_ship_list(&self, ships:Vec<mvc::ShipModel>){
        let j = serde_json::to_string_pretty(&ships).unwrap();
        fs::write("D:/rust/swn_helper/output/text.json", j).unwrap();
    }

    pub fn load_ship_list(&self){
        let files = FileDialog::new().pick_file();

        match files {
            None => {},
            Some(path) => {
                match fs::read_to_string(path) {
                    Err(_) => {},
                    Ok(contents) => {
                        let ship_list:Result<Vec<ShipModel>> = serde_json::from_str(&contents);
                        self.load_ship_list_callback.invoke(&ship_list.unwrap());
                    }
                }
            }
        }
    }

    pub fn load_hull_list(&self){
        let file = FileDialog::new().pick_file();
        match file {
            None => {},
            Some(path) => {
                match fs::read_to_string(path) {
                    Err(_) => {},
                    Ok(contents) => {
                        let ship_hull_list:Result<Vec<ShipHullModel>> = serde_json::from_str(&contents);
                        self.load_ship_hull_list_callback.invoke(&ship_hull_list.unwrap());
                    }
                }
            }
        }
    }

    pub fn on_load_ship_list(&self, mut callback: impl FnMut(&Vec<mvc::ShipModel>) + 'static){
        self.load_ship_list_callback.on(move|ship_list|{
            callback(ship_list);
        })
    }

    pub fn on_load_ship_hull_list(&self, mut callback: impl FnMut(&Vec<mvc::ShipHullModel>) + 'static){
        self.load_ship_hull_list_callback.on(move|ship_hull_list|{
            callback(ship_hull_list);
        })
    }

    
}
