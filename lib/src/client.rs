use crate::commands;
use crate::error::RobinError;
use crate::model;

pub struct RobinClient;

impl RobinClient {
    pub fn new() -> Self {
        Self {}
    }

    pub fn originators(&self) -> Result<Vec<model::Originator>, RobinError> {
        commands::get_originators()
    }

    /*pub fn neighbors(&self) -> Result<Vec<Neighbor>, RobinError> {
        commands::get_neighbors()
    }

    pub fn gateways(&self) -> Result<Vec<Gateway>, RobinError> {
        commands::get_gateways()
    }*/

    // ...
}
