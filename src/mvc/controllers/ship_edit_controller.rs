use std::rc::Rc;

//use crate::mvc::traits::ShipModel;

#[derive(Clone)]
pub struct ShipEditController{
    tmp: String,
    // ship: Rc<dyn ShipModel>,
}

impl ShipEditController{
    pub fn new() -> Self{
        Self{ tmp:"Test".into()}
    }
}