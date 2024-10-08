use slint::{Model, ModelNotify, ModelRc, ModelTracker};
use std::default;
use std::rc::Rc;
use std::sync::Arc;

use crate::mvc::{self, ShipFittingsModel, ShipModel, ShipWeaponModel};

use crate::{callback, Callback};
//use crate::mvc::traits::ShipModel;

#[derive(Clone)]
pub struct ShipEditController {
    current_ship:ShipModel,
    ship_hull_model: ShipHullModel,
    pub ship_weapon_model: Arc<Vec<mvc::ShipWeaponModel>>,
    pub ship_fittings_model: Arc<Vec<mvc::ShipFittingsModel>>,
    start_edit_ship_callback: Rc<Callback<ShipModel, ()>>,
    ship_updated_callback: Rc<callback<ShipModel,()>>,
}

impl ShipEditController {
    pub fn new(
        repo: impl mvc::traits::ShipHullRepository + 'static,
        ship_weapon_model: Vec<mvc::ShipWeaponModel>,
        ship_fittings_model: Vec<mvc::ShipFittingsModel>,
    ) -> Self {
        Self {
            current_ship: ShipModel::default(),
            ship_hull_model: ShipHullModel::new(repo),
            ship_weapon_model: Arc::from(ship_weapon_model),
            ship_fittings_model: Arc::from(ship_fittings_model),
            start_edit_ship_callback: Rc::new(Callback::default()),
            ship_updated_callback: Rc::new(Callback::default()),
        }
    }

    pub fn start_editing_ship(&self, new_ship: ShipModel) {
        self.start_edit_ship_callback.invoke(&new_ship);
    }

    pub fn on_start_editing_ship(&self, mut callback: impl FnMut(&ShipModel) + 'static) {
        self.start_edit_ship_callback.on(move |new_ship| {
            callback(new_ship);
        });
    }

    pub fn set_hull_model(&mut self, index:usize) -> Result<bool,()>{
        match self.ship_hull_model.get_hull(index){
            Some(hull) => self.current_ship.hull = hull,
            None => println!("No hull found")
        }
        Ok(true)
    }
    pub fn ship_hull_model(&self) -> ModelRc<mvc::ShipHullModel> {
        ModelRc::new(self.ship_hull_model.clone())
    }
    pub fn get_ship_hull(&self, index: usize) -> Option<mvc::ShipHullModel> {
        self.ship_hull_model.get_hull(index)
    }

    pub fn remove_ship_hull(&self, index: usize) {
        self.ship_hull_model.remove_ship_hull(index);
    }

    pub fn create_ship_hull(&self, new_ship_hull: mvc::models::ShipHullModel) {
        self.ship_hull_model.push_ship_hull(new_ship_hull);
    }

    //Search through the weapon list, building an array of the indecies where those weapons exist in the main list array for displaying
    pub fn get_weapon_indecies(
        ship_weapon_model: &Vec<ShipWeaponModel>,
        weapons: &[&str],
    ) -> Vec<i32> {
        //todo: this shouldn't be unwrapped here. At this point if the weapon is not found it would return an error
        weapons
            .iter()
            .map(|&weapon| {
                ship_weapon_model
                    .iter()
                    .position(|w| w.name == weapon)
                    .unwrap() as i32
            })
            .collect()
    }

    //Search through the weapon list, building an array of the indecies where those weapons exist in the main list array for displaying
    pub fn get_fitting_indecies(
        ship_fitting_model: &Vec<ShipFittingsModel>,
        fittings: &[&str],
    ) -> Vec<i32> {
        //todo: this shouldn't be unwrapped here. At this point if the weapon is not found it would return an error
        fittings
            .iter()
            .map(|&fitting| {
                ship_fitting_model
                    .iter()
                    .position(|f| f.name == fitting)
                    .unwrap() as i32
            })
            .collect()
    }
}

#[derive(Clone)]
struct ShipHullModel {
    repo: Rc<dyn mvc::traits::ShipHullRepository>,
    notify: Rc<ModelNotify>,
}
impl ShipHullModel {
    fn new(repo: impl mvc::traits::ShipHullRepository + 'static) -> Self {
        Self {
            repo: Rc::new(repo),
            notify: Rc::new(Default::default()),
        }
    }

    fn get_hull(&self, index: usize) -> Option<mvc::ShipHullModel> {
        self.repo.get_ship_hull(index)
    }
    fn remove_ship_hull(&self, index: usize) {
        if !self.repo.remove_ship_hull(index) {
            return;
        }
        self.notify.row_removed(index, 1)
    }

    fn push_ship_hull(&self, hull: mvc::ShipHullModel) {
        if !self.repo.push_ship_hull(hull) {
            return;
        }
        self.notify.row_added(self.row_count() - 1, 1);
    }
}

impl Model for ShipHullModel {
    type Data = mvc::ShipHullModel;

    fn row_count(&self) -> usize {
        self.repo.ship_hull_count()
    }

    fn row_data(&self, row: usize) -> Option<Self::Data> {
        self.repo.get_ship_hull(row)
    }

    fn model_tracker(&self) -> &dyn ModelTracker {
        self.notify.as_ref()
    }
}
