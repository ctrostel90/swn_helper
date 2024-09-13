use slint::*;
use std::rc::Rc;

use crate::{
    mvc::{
        MenuController, AppConfig, ShipModel,
    },
    ui,
};

fn connect_with_controller(
    view_handle: &ui::MainWindow,
    controller: &AppConfig,
    connect_adapter_controller: impl FnOnce(AppConfig) + 'static,
) {
    connect_adapter_controller(view_handle.global(), controller.clone());
}

pub fn connect_controller(view_handle: &ui::MainWindow, controller: AppConfig) {
    connect_with_controller_and_ship_list(view_handle, &controller, &ship_list_controller, {
        move |adapter, controller, ship_list_controller,| {
            adapter.on_save_clicked(move || {
                controller.save_ship_list(ship_list_controller.get_all_ships()); 
            });
        }
    });
}