use crate::mvc;

pub trait ShipHullRepository {
    fn ship_hull_count(&self) -> usize;
    fn get_ship_hull(&self, index: usize) -> Option<mvc::ShipHullModel>;
    fn remove_ship_hull(&self, index: usize) -> bool;
    fn clear_ship_hulls(&self) -> bool;
    fn push_ship_hull(&self, task: mvc::ShipHullModel) -> bool;
}
