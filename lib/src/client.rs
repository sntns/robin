use crate::commands;
use crate::error::RobinError;
use crate::model;

pub struct RobinClient;

impl RobinClient {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn originators(&self, mesh_if: &str) -> Result<Vec<model::Originator>, RobinError> {
        commands::get_originators(mesh_if).await
    }

    pub async fn algo_name(&self, mesh_if: &str) -> Result<String, RobinError> {
        commands::get_algo_name(mesh_if).await
    }

    pub async fn if_nametoindex(&self, ifname: &str) -> Result<u32, RobinError> {
        commands::if_nametoindex(ifname).await
    }

    pub async fn if_indextoname(&self, ifindex: u32) -> Result<String, RobinError> {
        commands::if_indextoname(ifindex).await
    }

    pub async fn gateways(&self, mesh_if: &str) -> Result<Vec<model::Gateway>, RobinError> {
        commands::get_gateways_list(mesh_if).await
    }

    pub async fn get_gw_mode(&self, mesh_if: &str) -> Result<model::GatewayInfo, RobinError> {
        commands::get_gateway(mesh_if).await
    }

    pub async fn set_gw_mode(
        &self,
        mode: model::GwMode,
        down: Option<u32>,
        up: Option<u32>,
        sel_class: Option<u32>,
        mesh_if: &str,
    ) -> Result<(), RobinError> {
        commands::set_gateway(mode, down, up, sel_class, mesh_if).await
    }

    pub async fn transglobal(
        &self,
        mesh_if: &str,
    ) -> Result<Vec<model::TransglobalEntry>, RobinError> {
        commands::get_transglobal(mesh_if).await
    }

    pub async fn translocal(
        &self,
        mesh_if: &str,
    ) -> Result<Vec<model::TranslocalEntry>, RobinError> {
        commands::get_translocal(mesh_if).await
    }

    pub async fn neighbors(&self, mesh_if: &str) -> Result<Vec<model::Neighbor>, RobinError> {
        commands::get_neighbors(mesh_if).await
    }

    pub async fn get_interface(&self, mesh_if: &str) -> Result<Vec<model::Interface>, RobinError> {
        commands::get_interfaces(mesh_if).await
    }

    pub async fn set_interface(
        &self,
        iface: &str,
        mesh_if: Option<&str>,
    ) -> Result<(), RobinError> {
        commands::set_interface(iface, mesh_if).await
    }

    pub async fn create_interface(
        &self,
        mesh_if: &str,
        routing_algo: Option<&str>,
    ) -> Result<(), RobinError> {
        commands::create_interface(mesh_if, routing_algo).await
    }

    pub async fn destroy_interface(&self, mesh_if: &str) -> Result<(), RobinError> {
        commands::destroy_interface(mesh_if).await
    }

    pub async fn count_interfaces(&self, mesh_if: &str) -> Result<u32, RobinError> {
        commands::count_interfaces(mesh_if).await
    }

    pub async fn aggregation(&self, mesh_if: &str) -> Result<bool, RobinError> {
        commands::get_aggregation(mesh_if).await
    }

    pub async fn set_aggregation(&self, mesh_if: &str, val: bool) -> Result<(), RobinError> {
        commands::set_aggregation(mesh_if, val).await
    }

    pub async fn ap_isolation(&self, mesh_if: &str) -> Result<bool, RobinError> {
        commands::get_ap_isolation(mesh_if).await
    }

    pub async fn set_ap_isolation(&self, mesh_if: &str, val: bool) -> Result<(), RobinError> {
        commands::set_ap_isolation(mesh_if, val).await
    }

    pub async fn bridge_loop_avoidance(&self, mesh_if: &str) -> Result<bool, RobinError> {
        commands::get_bridge_loop_avoidance(mesh_if).await
    }

    pub async fn set_bridge_loop_avoidance(
        &self,
        mesh_if: &str,
        val: bool,
    ) -> Result<(), RobinError> {
        commands::set_bridge_loop_avoidance(mesh_if, val).await
    }
}
