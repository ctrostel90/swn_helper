use slint::*;
use std::rc::Rc;

use crate::{
    mvc::{
        MenuController, ShipListController, ShipModel,
    },
    ui,
};

fn connect_with_controller_and_ship_list(
    view_handle: &ui::MainWindow,
    controller: &MenuController,
    ship_list_controller: &ShipListController,
    connect_adapter_controller: impl FnOnce(ui::MenuBarAdapter, MenuController, ShipListController) + 'static,
) {
    connect_adapter_controller(view_handle.global::<ui::MenuBarAdapter>(), controller.clone(),ship_list_controller.clone());
}

pub fn connect_ship_list_controller(view_handle: &ui::MainWindow, controller: MenuController, ship_list_controller: ShipListController) {
    connect_with_controller_and_ship_list(view_handle, &controller, &ship_list_controller, {
        move |adapter, controller, ship_list_controller,| {
            adapter.on_save_clicked(move || {
                controller.save_ship_list(ship_list_controller.get_all_ships()); 
            });
        }
    });

    connect_with_controller_and_ship_list(view_handle, &controller, &ship_list_controller, {
        move |adapter, controller, ship_list_controller,| {
            adapter.on_load_clicked(move || {
                controller.load_ship_list(); 
            });
        }
    });

    connect_with_controller_and_ship_list(view_handle, &controller, &ship_list_controller, {
        move |adapter, controller, ship_list_controller,| {
            adapter.on_load_hulls_clicked(move || {
                controller.load_hull_list(); 
            });
        }
    });

}

// fn map_ship_data_to_ship_model(ship_data: ui::ShipBuilderData) -> ShipModel{
//     ShipModel{
//         name: ship_data.ship_name.into(),
//         hull: ship_data.ship_hull_type.into(),
//         hp: ship_data.ship_hp as i64,
//         npc_cp_count: ship_data.ship_npc_cp as i64,
//         weapon_one: ship_data.weapon_one.into(),
//         fitting_one: ship_data.fitting_one.into(),
//     }
// }

