use std::rc::Rc;

use crate::mvc::ShipModel;

use crate::Callback;
//use crate::mvc::traits::ShipModel;

#[derive(Clone)]
pub struct ShipEditController{
    start_edit_ship_callback:Rc<Callback<ShipModel,()>>,
}

impl ShipEditController{
    pub fn new() -> Self{
        Self{ 
            start_edit_ship_callback: Rc::new(Callback::default())
        }
    }

    pub fn start_editing_ship(&self, new_ship:ShipModel){
        self.start_edit_ship_callback.invoke(&new_ship);
    }

    pub fn on_start_editing_ship(&self, mut callback: impl FnMut(&ShipModel) + 'static){
        self.start_edit_ship_callback.on(move|new_ship|{
            callback(new_ship);
        });
    }
}