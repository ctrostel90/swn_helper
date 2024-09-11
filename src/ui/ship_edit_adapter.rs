use slint::*;
use std::rc::Rc;

use crate::{
    mvc::{
        ShipEditController, ShipListController, ShipModel,MenuController,ShipHullModel,
    },
    ui,
};

fn connect_with_controller(
    view_handle: &ui::MainWindow,
    controller: &ShipEditController,
    connect_adapter_controller: impl FnOnce(ui::ShipBuilderAdapter, ShipEditController) + 'static,
) {
    connect_adapter_controller(view_handle.global::<ui::ShipBuilderAdapter>(), controller.clone());
}

pub fn connect(view_handle: &ui::MainWindow, controller: ShipEditController) {
    view_handle
        .global::<ui::ShipHullComboAdapter>()
        .set_model(Rc::new(MapModel::new(controller.ship_hull_model(), map_hull_to_item)).into());

    controller.on_start_editing_ship({
        let view_handle = view_handle.as_weak();

        move |new_ship| {
            view_handle
            .unwrap()
            .global::<ui::ShipBuilderAdapter>()
            .invoke_set_ship(map_ship_model_to_ship_data(new_ship.clone()));
        }  
    });

}

fn connect_with_menu_controller(
    view_handle: &ui::MainWindow,
    controller: &ShipEditController,
    menu_controller: &MenuController,
    connect_adapter_controller: impl FnOnce(ui::ShipBuilderAdapter, ShipEditController, MenuController) + 'static,
) {
    connect_adapter_controller(view_handle.global::<ui::ShipBuilderAdapter>(), controller.clone(),menu_controller.clone());
}

pub fn connect_menu_controller(view_handle: &ui::MainWindow, controller: ShipEditController, menu_controller: MenuController) {
    // sets a mapped list of the task items to the ui
    menu_controller.on_load_ship_hull_list({
        move |new_ship_hull_list|{
            //I'm POSITIVE this is not a good way to do this but, here we are
            //Remove all the ships
            for index in 0..controller.ship_hull_model().row_count(){
                controller.remove_ship_hull(index);
            }
            //add all the new ships
            for index in 0..new_ship_hull_list.len(){
                controller.create_ship_hull(new_ship_hull_list[index].clone());
            }
        }
    });
}

fn connect_with_ship_list_controller(
    view_handle: &ui::MainWindow,
    controller: &ShipListController,
    connect_adapter_controller: impl FnOnce(ui::ShipBuilderAdapter, ShipListController) + 'static,
) {
    connect_adapter_controller(view_handle.global::<ui::ShipBuilderAdapter>(), controller.clone());
}

// pub fn connect_ship_list_controller(view_handle: &ui::MainWindow, controller: ShipListController) {
//     connect_with_ship_list_controller(view_handle, &controller, {
//         move |adapter, controller| {
//             adapter.on_create(move |new_ship| {
//                 controller.create_ship(map_ship_data_to_ship_model(new_ship))
//             })
//         }
//     });

// }
fn map_hull_to_item(ship_hull:ShipHullModel)-> ModelRc<slint::SharedString>{
    ModelRc::new(Rc::new(VecModel::from(vec![
        slint::SharedString::from(&ship_hull.name),
        slint::format!("{}",ship_hull.cost),
        slint::format!("{}",ship_hull.speed),
        slint::format!("{}",ship_hull.armor),
        slint::format!("{}",ship_hull.hp),
        slint::SharedString::from(&ship_hull.get_crew()),
        slint::format!("{}",ship_hull.ac),
        slint::format!("{}",ship_hull.power),
        slint::format!("{}",ship_hull.mass),        
        slint::format!("{}",ship_hull.hard_points),
        slint::SharedString::from(&ship_hull.class),
    ])))

}
fn map_ship_model_to_ship_data(ship_model: ShipModel) -> ui::ShipBuilderData{
    ui::ShipBuilderData{
        ship_name: SharedString::from(&ship_model.name),
        ship_hull_type: SharedString::from(&ship_model.hull.name),
        ship_class: SharedString::from(&ship_model.hull.class),
        ship_hp: ship_model.hp as i32,
        ship_crew_min: ship_model.hull.crew_minimum as i32,
        ship_crew_max: ship_model.hull.crew_maximum as i32,
        ship_armor: ship_model.get_armor() as i32,
        ship_ac: ship_model.get_ac() as i32,
        ship_power: ship_model.get_power() as i32,
        ship_mass: ship_model.get_mass() as i32,
        ship_npc_cp: ship_model.npc_cp_count as i32,
        ship_speed: ship_model.get_speed() as i32,
        weapon_one: SharedString::from(&ship_model.weapon_one),
        fitting_one: SharedString::from(&ship_model.fitting_one),
    }
}