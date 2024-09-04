use slint::*;
use std::rc::Rc;

use crate::{
    mvc::{ShipListController, ShipModel},
    ui,
};

// a helper function to make adapter and controller connection a little bit easier
pub fn connect_with_controller(
    view_handle: &ui::MainWindow,
    controller: &ShipListController,
    connect_adapter_controller: impl FnOnce(ui::ShipListAdapter, ShipListController) + 'static,
) {
    connect_adapter_controller(view_handle.global::<ui::ShipListAdapter>(), controller.clone());
}

// one place to implement connection between adapter (view) and controller
pub fn connect(view_handle: &ui::MainWindow, controller: ShipListController) {
    // sets a mapped list of the task items to the ui
    view_handle
        .global::<ui::ShipListAdapter>()
        .set_ships(Rc::new(MapModel::new(controller.ship_model(), map_ship_to_item)).into());

   
    connect_with_controller(view_handle, &controller, {
        move |adapter, controller| {
            adapter.on_remove_ship(move |index| {
                controller.remove_ship(index as usize);
            })
        }
    });
}

fn map_ship_to_item(ship: ShipModel) -> ui::ShipListViewItem {
    ui::ShipListViewItem {
        ship_name: ship.name.into(),
        ship_hull_type: ship.hull.into(),
        ship_class: ship.class.into(),
        ship_hp: ship.hp as i32,
        ship_crew_min: ship.crew_minimum as i32,
        ship_crew_max: ship.crew_maximum as i32,
        ..Default::default()
    }
}
