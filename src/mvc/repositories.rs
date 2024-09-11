pub mod traits;
use crate::mvc::models::{ShipModel,ShipHullModel};

mod mock_ship_repository;
pub use mock_ship_repository::*;

mod mock_ship_hull_repository;
pub use mock_ship_hull_repository::*;

pub fn ship_repo() -> impl traits::ShipRepository + Clone{
    MockShipRepository::new(vec![
        ShipModel{
            name: "Shipperoo".into(),
            hull: ShipHullModel{
                name:"Free Merchant".into(),
                cost: 100_000,
                speed:3,
                armor:5,
                hp:8,
                hard_points:4,
                crew_minimum:1,
                crew_maximum:10,
                ac:3,
                power:10,
                mass:500,
                class:"Free Merchant".into(),
            },
            hp:12,
            npc_cp_count: 12,
            weapon_one: "Multifocal Laser".into(),
            fitting_one: "Spike Drive-1".into(),
        },]
    )
}

pub fn ship_hull_repo() -> impl traits::ShipHullRepository + Clone{
    MockShipHullRepository::new(vec![
        ShipHullModel{
            name:"Free Merchant".into(),
            cost: 100_000,
            speed:3,
            hp:8,
            armor:5,
            hard_points:4,
            crew_minimum:1,
            crew_maximum:10,
            ac:3,
            power:10,
            mass:500,
            class:"Free Merchant".into(),
        },]
    )
}