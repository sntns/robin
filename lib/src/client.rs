use crate::commands;
use crate::error::RobinError;
use crate::model;

/// High-level client for interacting with the BATMAN-adv mesh network.
///
/// This struct provides async methods to query and manage mesh interfaces,
/// routing algorithms, gateways, neighbors, and translation tables.
///
/// All methods return a `Result` with either the requested data or a `RobinError`.
pub struct RobinClient;

impl RobinClient {
    /// Creates a new instance of `RobinClient`.
    pub fn new() -> Self {
        Self {}
    }

    /// Converts a network interface name to its corresponding index.
    pub async fn if_nametoindex(&self, ifname: &str) -> Result<u32, RobinError> {
        commands::if_nametoindex(ifname).await
    }

    /// Converts a network interface index to its corresponding name.
    pub async fn if_indextoname(&self, ifindex: u32) -> Result<String, RobinError> {
        commands::if_indextoname(ifindex).await
    }

    /// Retrieves the list of originators for the given mesh interface.
    pub async fn originators(&self, mesh_if: &str) -> Result<Vec<model::Originator>, RobinError> {
        commands::get_originators(mesh_if).await
    }

    /// Retrieves the list of gateways for the given mesh interface.
    pub async fn gateways(&self, mesh_if: &str) -> Result<Vec<model::Gateway>, RobinError> {
        commands::get_gateways_list(mesh_if).await
    }

    /// Gets the current gateway mode and configuration for the given mesh interface.
    pub async fn get_gw_mode(&self, mesh_if: &str) -> Result<model::GatewayInfo, RobinError> {
        commands::get_gateway(mesh_if).await
    }

    /// Sets the gateway mode and optional bandwidth/selection parameters for the mesh interface.
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

    /// Retrieves the global translation table entries for the mesh interface.
    pub async fn transglobal(
        &self,
        mesh_if: &str,
    ) -> Result<Vec<model::TransglobalEntry>, RobinError> {
        commands::get_transglobal(mesh_if).await
    }

    /// Retrieves the local translation table entries for the mesh interface.
    pub async fn translocal(
        &self,
        mesh_if: &str,
    ) -> Result<Vec<model::TranslocalEntry>, RobinError> {
        commands::get_translocal(mesh_if).await
    }

    /// Retrieves the list of neighbors for the given mesh interface.
    pub async fn neighbors(&self, mesh_if: &str) -> Result<Vec<model::Neighbor>, RobinError> {
        commands::get_neighbors(mesh_if).await
    }

    /// Retrieves the list of hard interfaces attached to the mesh interface.
    pub async fn get_interface(&self, mesh_if: &str) -> Result<Vec<model::Interface>, RobinError> {
        commands::get_interfaces(mesh_if).await
    }

    /// Adds or removes a hard interface from the mesh interface.
    pub async fn set_interface(
        &self,
        iface: &str,
        mesh_if: Option<&str>,
    ) -> Result<(), RobinError> {
        commands::set_interface(iface, mesh_if).await
    }

    /// Creates a new BATMAN-adv mesh interface with an optional routing algorithm.
    pub async fn create_interface(
        &self,
        mesh_if: &str,
        routing_algo: Option<&str>,
    ) -> Result<(), RobinError> {
        commands::create_interface(mesh_if, routing_algo).await
    }

    /// Destroys a BATMAN-adv mesh interface.
    pub async fn destroy_interface(&self, mesh_if: &str) -> Result<(), RobinError> {
        commands::destroy_interface(mesh_if).await
    }

    /// Counts the number of hard interfaces attached to the mesh interface.
    pub async fn count_interfaces(&self, mesh_if: &str) -> Result<u32, RobinError> {
        commands::count_interfaces(mesh_if).await
    }

    /// Checks whether packet aggregation is enabled on the mesh interface.
    pub async fn get_aggregation(&self, mesh_if: &str) -> Result<bool, RobinError> {
        commands::get_aggregation(mesh_if).await
    }

    /// Enables or disables packet aggregation on the mesh interface.
    pub async fn set_aggregation(&self, mesh_if: &str, val: bool) -> Result<(), RobinError> {
        commands::set_aggregation(mesh_if, val).await
    }

    /// Checks whether AP isolation is enabled on the mesh interface.
    pub async fn get_ap_isolation(&self, mesh_if: &str) -> Result<bool, RobinError> {
        commands::get_ap_isolation(mesh_if).await
    }

    /// Enables or disables AP isolation on the mesh interface.
    pub async fn set_ap_isolation(&self, mesh_if: &str, val: bool) -> Result<(), RobinError> {
        commands::set_ap_isolation(mesh_if, val).await
    }

    /// Checks whether bridge loop avoidance is enabled on the mesh interface.
    pub async fn get_bridge_loop_avoidance(&self, mesh_if: &str) -> Result<bool, RobinError> {
        commands::get_bridge_loop_avoidance(mesh_if).await
    }

    /// Enables or disables bridge loop avoidance on the mesh interface.
    pub async fn set_bridge_loop_avoidance(
        &self,
        mesh_if: &str,
        val: bool,
    ) -> Result<(), RobinError> {
        commands::set_bridge_loop_avoidance(mesh_if, val).await
    }

    /// Retrieves the system default routing algorithm for BATMAN-adv.
    pub async fn get_default_routing_algo(&self) -> Result<String, RobinError> {
        commands::get_default_routing_algo().await
    }

    /// Retrieves all active routing algorithms currently in use with their mesh interfaces.
    pub async fn get_active_routing_algos(&self) -> Result<Vec<(String, String)>, RobinError> {
        commands::get_active_routing_algos().await
    }

    /// Retrieves the list of all routing algorithms available on the system.
    pub async fn get_available_routing_algos(&self) -> Result<Vec<String>, RobinError> {
        commands::get_available_routing_algos().await
    }

    /// Sets the system default routing algorithm for BATMAN-adv.
    pub async fn set_default_routing_algo(&self, algo: &str) -> Result<(), RobinError> {
        commands::set_default_routing_algo(algo).await
    }
}
