
pub mod ui;
pub mod mvc;

pub use slint::*;
pub fn main(){
    let main_window = init();

    main_window.run().unwrap();
}

fn init() -> ui::MainWindow{
    let view_handle = ui::MainWindow::new().unwrap();

    let ship_list_controller = mvc::ShipListController::new(mvc::ship_repo());
    ui::ship_list_adapter::connect(&view_handle,ship_list_controller.clone());
    
    //let ship_edit_controller = mvc::ShipEditController::new();
    ui::ship_edit_adapter::connect_ship_list_controller(&view_handle, ship_list_controller.clone());

    view_handle
}