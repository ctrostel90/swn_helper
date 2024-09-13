use serde::{Serialize,Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShipFittingsModel {
    pub name: String,
    pub cost: String,
    pub power: String,
    pub mass: String,
    pub class:String,
    pub effect: String,
}

impl Default for ShipFittingsModel{
    fn default() -> Self {
        Self{
            name:"".into(),
            cost:"".into(),
            power:"".into(),
            mass:"".into(),
            class:"".into(),
            effect:"".into(),
        }
    }
}