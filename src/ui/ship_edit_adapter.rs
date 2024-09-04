use slint::*;

use crate::{
    mvc::{
        ShipEditController, ShipListController, ShipModel,
    },
    ui,
};

fn connect_with_ship_list_controller(
    view_handle: &ui::MainWindow,
    controller: &ShipListController,
    connect_adapter_controller: impl FnOnce(ui::ShipBuilderAdapter, ShipListController) + 'static,
) {
    connect_adapter_controller(view_handle.global::<ui::ShipBuilderAdapter>(), controller.clone());
}

pub fn connect_ship_list_controller(view_handle: &ui::MainWindow, controller: ShipListController) {
    connect_with_ship_list_controller(view_handle, &controller, {
        move |adapter, controller| {
            adapter.on_create(move |new_ship| {
                controller.create_ship(map_ship_data_to_ship_model(new_ship))
            })
        }
    });
}

fn map_ship_data_to_ship_model(ship_data: ui::ShipBuilderData) -> ShipModel{
    ShipModel{
        name: ship_data.ship_name.into(),
        hull: ship_data.ship_hull_type.into(),
        class: ship_data.ship_class.into(),
        hp: ship_data.ship_hp as i64,
        crew_minimum: ship_data.ship_crew_min as i64,
        crew_maximum: ship_data.ship_crew_max as i64,
        armor: ship_data.ship_armor as i64,
        ac: ship_data.ship_ac as i64,
        power: ship_data.ship_power as i64,
        mass: ship_data.ship_mass as i64,
        npc_cp_count: ship_data.ship_npc_cp as i64,
        speed: ship_data.ship_speed as i64,
        weapon_one: ship_data.weapon_one.into(),
        fitting_one: ship_data.fitting_one.into(),
    }
}