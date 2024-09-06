use slint::*;
use std::rc::Rc;

use crate::{
    mvc::{MenuController, ShipEditController, ShipListController, ShipModel},
    ui,
};

// a helper function to make adapter and controller connection a little bit easier
fn connect_with_controller(
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

fn connect_with_ship_edit_controller(
    view_handle: &ui::MainWindow,
    controller: &ShipListController,
    ship_edit_controller: &ShipEditController,
    connect_adapter_controller: impl FnOnce(ui::ShipListAdapter, ShipListController, ShipEditController) + 'static,
) {
    connect_adapter_controller(view_handle.global::<ui::ShipListAdapter>(), controller.clone(), ship_edit_controller.clone());
}
pub fn connect_ship_edit_controller(
    view_handle: &ui::MainWindow,
    controller: &ShipListController,
    ship_edit_controller: &ShipEditController,){
        connect_with_ship_edit_controller(view_handle, &controller, &ship_edit_controller, {
        move |adapter, controller,ship_edit_controller| {
            adapter.on_edit_ship(move |index| {
                ship_edit_controller.start_editing_ship(controller.get_ship(index as usize).unwrap());
            })
        }
        });
}

fn connect_with_menu_controller(
    view_handle: &ui::MainWindow,
    controller: &ShipListController,
    menu_controller: &MenuController,
    connect_adapter_controller: impl FnOnce(ui::ShipListAdapter, ShipListController, MenuController) + 'static,
) {
    connect_adapter_controller(view_handle.global::<ui::ShipListAdapter>(), controller.clone(),menu_controller.clone());
}

// one place to implement connection between adapter (view) and controller
pub fn connect_menu_controller(view_handle: &ui::MainWindow, controller: ShipListController, menu_controller: MenuController) {
    // sets a mapped list of the task items to the ui
    menu_controller.on_load_ship_list({
        move |new_ship_list|{
            //I'm POSITIVE this is not a good way to do this but, here we are
            for index in 0..controller.ship_model().row_count(){
                controller.remove_ship(index);
            }
            for index in 0..new_ship_list.len(){
                controller.create_ship(new_ship_list[index].clone());
            }
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
