
pub mod ui;
pub mod mvc;

mod callback;
pub use callback::*;
pub use slint::*;

pub fn main(){
    let main_window = init();

    main_window.run().unwrap();
}

fn init() -> ui::MainWindow{
    let view_handle = ui::MainWindow::new().unwrap();

    let startup_controller = mvc::AppConfig::new(
        "D:/rust/swn_helper/assets/imports/ship_fittings.json".into(),
        "D:/rust/swn_helper/assets/imports/ship_weapons.json".into(),
        "D:/rust/swn_helper/assets/imports/ship_hulls.json".into(),
    );
    let (ship_hull_model,ship_weapon_model) = startup_controller.startup();
    let ship_list_controller = mvc::ShipListController::new(mvc::ship_repo());
    ui::ship_list_adapter::connect(&view_handle,ship_list_controller.clone());
        
    let ship_edit_controller = mvc::ShipEditController::new(mvc::ship_hull_repo(),ship_weapon_model);
    ui::ship_edit_adapter::connect(&view_handle, ship_edit_controller.clone());
    //ui::ship_edit_adapter::connect_ship_list_controller(&view_handle, ship_list_controller.clone());
    ui::ship_list_adapter::connect_ship_edit_controller(&view_handle, &ship_list_controller, &ship_edit_controller);

    let menu_controller = mvc::MenuController::new();
    ui::menu_adapter::connect_ship_list_controller(&view_handle, menu_controller.clone(), ship_list_controller.clone());
    ui::ship_list_adapter::connect_menu_controller(&view_handle, ship_list_controller.clone(), menu_controller.clone());
    ui::ship_edit_adapter::connect_menu_controller(&view_handle, ship_edit_controller.clone(),menu_controller.clone());
    view_handle
}