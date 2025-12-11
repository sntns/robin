use crate::commands;
use crate::error::RobinError;
use crate::model;

pub struct RobinClient;

impl RobinClient {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn originators(&self) -> Result<Vec<model::Originator>, RobinError> {
        commands::get_originators().await
    }

    pub async fn gateways(&self) -> Result<Vec<model::Gateway>, RobinError> {
        commands::get_gateways_list().await
    }

    pub async fn gateway(&self) -> Result<model::GatewayInfo, RobinError> {
        commands::get_gateway().await
    }

    // ...
}
