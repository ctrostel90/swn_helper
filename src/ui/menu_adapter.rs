use slint::*;

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

fn map_ship_model_to_ship_data(ship_model: ShipModel) -> ui::ShipBuilderData{
    ui::ShipBuilderData{
        ship_name: ship_model.name.into(),
        ship_hull_type: ship_model.hull.into(),
        ship_class: ship_model.class.into(),
        ship_hp: ship_model.hp as i32,
        ship_crew_min: ship_model.crew_minimum as i32,
        ship_crew_max: ship_model.crew_maximum as i32,
        ship_armor: ship_model.armor as i32,
        ship_ac: ship_model.ac as i32,
        ship_power: ship_model.power as i32,
        ship_mass: ship_model.mass as i32,
        ship_npc_cp: ship_model.npc_cp_count as i32,
        ship_speed: ship_model.speed as i32,
        weapon_one: ship_model.weapon_one.into(),
        fitting_one: ship_model.fitting_one.into(),
    }
}