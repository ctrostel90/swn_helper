use std::rc::Rc;
use slint::{Model,ModelNotify,ModelRc,ModelTracker,};

use crate::mvc::{self, ShipModel};

use crate::Callback;
//use crate::mvc::traits::ShipModel;

#[derive(Clone)]
pub struct ShipEditController{
    ship_hull_model:ShipHullModel,
    start_edit_ship_callback:Rc<Callback<ShipModel,()>>,
}

impl ShipEditController{
    pub fn new(repo: impl mvc::traits::ShipHullRepository + 'static) -> Self{
        Self{ 
            ship_hull_model: ShipHullModel::new(repo),
            start_edit_ship_callback: Rc::new(Callback::default())
        }
    }

    pub fn start_editing_ship(&self, new_ship:ShipModel){
        self.start_edit_ship_callback.invoke(&new_ship);
    }

    pub fn on_start_editing_ship(&self, mut callback: impl FnMut(&ShipModel) + 'static){
        self.start_edit_ship_callback.on(move|new_ship|{
            callback(new_ship);
        });
    }

    pub fn ship_hull_model(&self) -> ModelRc<mvc::ShipHullModel> {
        ModelRc::new(self.ship_hull_model.clone())
    }
    pub fn get_ship_hull(&self, index: usize) -> Option<mvc::ShipHullModel>{
        self.ship_hull_model.get_hull(index)
    }

    pub fn remove_ship_hull(&self, index: usize) {
        self.ship_hull_model.remove_ship_hull(index);
    }

    pub fn create_ship_hull(&self, new_ship_hull:mvc::models::ShipHullModel) {
        self.ship_hull_model.push_ship_hull(new_ship_hull);
    }

}

#[derive(Clone)]
struct ShipHullModel{
    repo: Rc<dyn mvc::traits::ShipHullRepository>,
    notify:Rc<ModelNotify>,
}
impl ShipHullModel {
    fn new(repo: impl mvc::traits::ShipHullRepository + 'static) -> Self {
        Self { repo: Rc::new(repo), notify: Rc::new(Default::default()) }
    }

    fn get_hull(&self,index:usize) -> Option<mvc::ShipHullModel>{
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