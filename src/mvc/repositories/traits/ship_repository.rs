use crate::mvc;

pub trait ShipRepository {
    fn ship_count(&self) -> usize;
    fn get_ship(&self, index: usize) -> Option<mvc::ShipModel>;
    fn remove_ship(&self, index: usize) -> bool;
    fn push_ship(&self, task: mvc::ShipModel) -> bool;
    fn get_all_ships(&self) -> Vec<mvc::ShipModel>;
}
