use std::{cell::RefCell, rc::Rc};

use super::traits;
use crate::mvc;

#[derive(Clone)]
pub struct MockShipRepository {
    ships: Rc<RefCell<Vec<mvc::ShipModel>>>,
}

impl MockShipRepository {
    pub fn new(ships: Vec<mvc::ShipModel>) -> Self {
        Self { ships: Rc::new(RefCell::new(ships)) }
    }
}

impl traits::ShipRepository for MockShipRepository {
    fn ship_count(&self) -> usize {
        self.ships.borrow().len()
    }

    fn get_ship(&self, index: usize) -> Option<mvc::ShipModel> {
        self.ships.borrow().get(index).cloned()
    }

    fn remove_ship(&self, index: usize) -> bool {
        if index < self.ships.borrow().len() {
            self.ships.borrow_mut().remove(index);
            return true;
        }

        false
    }

    fn push_ship(&self, ship: mvc::ShipModel) -> bool {
        self.ships.borrow_mut().push(ship);
        true
    }
    fn get_all_ships(&self) -> Vec<mvc::ShipModel> {
        let mut tmp = Vec::<mvc::ShipModel>::new();
        for index in 0..self.ship_count(){
            tmp.push(self.get_ship(index).unwrap());
        }
        tmp
    }

    fn clear_ships(&self) -> bool {
        self.ships.borrow_mut().clear();
        true
    }
}
