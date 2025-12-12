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

    pub async fn get_gw_mode(&self) -> Result<model::GatewayInfo, RobinError> {
        commands::get_gateway().await
    }

    pub async fn set_gw_mode(
        &self,
        mode: model::GwMode,
        down: Option<u32>,
        up: Option<u32>,
        sel_class: Option<u32>,
    ) -> Result<(), RobinError> {
        commands::set_gateway(mode, down, up, sel_class).await
    }

    pub async fn transglobal(&self) -> Result<Vec<model::TransglobalEntry>, RobinError> {
        commands::get_transglobal().await
    }

    pub async fn translocal(&self) -> Result<Vec<model::TranslocalEntry>, RobinError> {
        commands::get_translocal().await
    }

    pub async fn neighbors(&self) -> Result<Vec<model::Neighbor>, RobinError> {
        commands::get_neighbors().await
    }

    // ...
}
