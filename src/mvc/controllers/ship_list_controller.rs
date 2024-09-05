use std::rc::Rc;

use slint::Model;
use slint::ModelNotify;
use slint::ModelRc;
use slint::ModelTracker;

use crate::mvc;

#[derive(Clone)]
pub struct ShipListController {
    ship_model: ShipModel,
}
impl ShipListController{
    pub fn new(repo: impl mvc::traits::ShipRepository + 'static) -> Self {
        Self {
            ship_model: ShipModel::new(repo)
        }
    }

    pub fn ship_model(&self) -> ModelRc<mvc::ShipModel> {
        ModelRc::new(self.ship_model.clone())
    }

    pub fn get_ship(&self, index: usize) -> Option<mvc::ShipModel>{
        self.ship_model.get_ship(index)
    }

    pub fn remove_ship(&self, index: usize) {
        self.ship_model.remove_ship(index);
    }

    pub fn create_ship(&self, new_ship:mvc::models::ShipModel) {
        self.ship_model.push_ship(new_ship);
    }
}

#[derive(Clone)]
struct ShipModel{
    repo: Rc<dyn mvc::traits::ShipRepository>,
    notify:Rc<ModelNotify>,
}

impl ShipModel {
    fn new(repo: impl mvc::traits::ShipRepository + 'static) -> Self {
        Self { repo: Rc::new(repo), notify: Rc::new(Default::default()) }
    }

    fn get_ship(&self,index:usize) -> Option<mvc::ShipModel>{
        self.repo.get_ship(index)
    }
    fn remove_ship(&self, index: usize) {
        if !self.repo.remove_ship(index) {
            return;
        }

        self.notify.row_removed(index, 1)
    }

    fn push_ship(&self, ship: mvc::ShipModel) {
        if !self.repo.push_ship(ship) {
            return;
        }

        self.notify.row_added(self.row_count() - 1, 1);
    }
}

impl Model for ShipModel {
    type Data = mvc::ShipModel;

    fn row_count(&self) -> usize {
        self.repo.ship_count()
    }

    fn row_data(&self, row: usize) -> Option<Self::Data> {
        self.repo.get_ship(row)
    }

    fn model_tracker(&self) -> &dyn ModelTracker {
        self.notify.as_ref()
    }
}