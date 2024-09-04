pub mod traits;
use crate::mvc::models::ShipModel;

mod mock_ship_repository;
pub use mock_ship_repository::*;

pub fn ship_repo() -> impl traits::ShipRepository + Clone{
    MockShipRepository::new(vec![
        ShipModel{
            name: "Shipperoo".into(),
            hull: "Free Merchant".into(),
            class: "Frigate".into(),
            hp: 15,
            crew_minimum: 1,
            crew_maximum: 10,
            armor: 5,
            ac: 3,
            power: 10,
            mass: 500,
            npc_cp_count: 12,
            speed: 3,
            weapon_one: "Multifocal Laser".into(),
            fitting_one: "Spike Drive-1".into(),
        },]
    )
}