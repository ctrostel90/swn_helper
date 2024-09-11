use std::{cell::RefCell, rc::Rc};

use super::traits;
use crate::mvc;

#[derive(Clone)]
pub struct MockShipHullRepository {
    ship_hulls: Rc<RefCell<Vec<mvc::ShipHullModel>>>,
}

impl MockShipHullRepository {
    pub fn new(ship_hulls: Vec<mvc::ShipHullModel>) -> Self {
        Self { ship_hulls: Rc::new(RefCell::new(ship_hulls)) }
    }
}

impl traits::ShipHullRepository for MockShipHullRepository {
    fn ship_hull_count(&self) -> usize {
        self.ship_hulls.borrow().len()
    }

    fn get_ship_hull(&self, index: usize) -> Option<mvc::ShipHullModel> {
        self.ship_hulls.borrow().get(index).cloned()
    }

    fn remove_ship_hull(&self, index: usize) -> bool {
        if index < self.ship_hulls.borrow().len() {
            self.ship_hulls.borrow_mut().remove(index);
            return true;
        }

        false
    }

    fn push_ship_hull(&self, ship: mvc::ShipHullModel) -> bool {
        self.ship_hulls.borrow_mut().push(ship);
        true
    }
    fn clear_ship_hulls(&self) -> bool {
        self.ship_hulls.borrow_mut().clear();
        true
    }
}
