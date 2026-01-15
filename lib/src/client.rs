use crate::commands;
use crate::error::RobinError;
use crate::model;

/// High-level client for interacting with the BATMAN-adv mesh network.
///
/// `RobinClient` provides async methods to query and manage mesh interfaces,
/// translation tables, routing algorithms, gateways, and neighbors.
///
/// All methods return a `Result` containing either the requested data or a `RobinError`.
///
/// # Example
///
/// ```no_run
/// use robin::RobinClient;
/// # async fn example() -> Result<(), robin::RobinError> {
/// let client = RobinClient::new();
/// let mesh_if = "bat0";
///
/// // Get all neighbors
/// let neighbors = client.neighbors(mesh_if).await?;
///
/// // Print active interfaces
/// let interfaces = client.get_interface(mesh_if).await?;
/// for iface in interfaces {
///     println!("{}: {}", iface.ifname, if iface.active { "active" } else { "inactive" });
/// }
/// # Ok(())
/// # }
/// ```
pub struct RobinClient;

impl RobinClient {
    /// Creates a new instance of `RobinClient`.
    ///
    /// # Example
    ///
    /// ```
    /// use robin::RobinClient;
    /// let client = RobinClient::new();
    /// ```
    pub fn new() -> Self {
        Self {}
    }

    /// Converts a network interface name to its corresponding index.
    ///
    /// # Arguments
    /// * `ifname` - Name of the network interface (e.g., "wlan0").
    ///
    /// # Example
    ///
    /// ```
    /// let idx = client.if_nametoindex("bat0").await?;
    /// println!("Interface index: {}", idx);
    /// ```
    pub async fn if_nametoindex(&self, ifname: &str) -> Result<u32, RobinError> {
        commands::if_nametoindex(ifname).await
    }

    /// Converts a network interface index to its corresponding name.
    ///
    /// # Arguments
    /// * `ifindex` - Interface index.
    ///
    /// # Example
    ///
    /// ```
    /// let ifname = client.if_indextoname(3).await?;
    /// println!("Interface name: {}", ifname);
    /// ```
    pub async fn if_indextoname(&self, ifindex: u32) -> Result<String, RobinError> {
        commands::if_indextoname(ifindex).await
    }

    /// Retrieves the list of originators for the given mesh interface.
    ///
    /// # Example
    ///
    /// ```
    /// let originators = client.originators("bat0").await?;
    /// for o in originators {
    ///     println!("Originator: {}", o.originator);
    /// }
    /// ```
    pub async fn originators(&self, mesh_if: &str) -> Result<Vec<model::Originator>, RobinError> {
        commands::get_originators(mesh_if).await
    }

    /// Retrieves the list of gateways for the given mesh interface.
    ///
    /// # Example
    ///
    /// ```
    /// let gateways = client.gateways("bat0").await?;
    /// for g in gateways {
    ///     println!("Gateway: {}", g.mac_addr);
    /// }
    /// ```
    pub async fn gateways(&self, mesh_if: &str) -> Result<Vec<model::Gateway>, RobinError> {
        commands::get_gateways_list(mesh_if).await
    }

    /// Gets the current gateway mode and configuration for the mesh interface.
    ///
    /// # Example
    ///
    /// ```
    /// let gw_info = client.get_gw_mode("bat0").await?;
    /// println!("Gateway mode: {:?}", gw_info.mode);
    /// ```
    pub async fn get_gw_mode(&self, mesh_if: &str) -> Result<model::GatewayInfo, RobinError> {
        commands::get_gateway(mesh_if).await
    }

    /// Sets the gateway mode and optional bandwidth/selection parameters for the mesh interface.
    ///
    /// # Arguments
    /// * `mode` - Gateway mode (`Off`, `Client`, `Server`)
    /// * `down` - Optional downlink bandwidth in kbit/s
    /// * `up` - Optional uplink bandwidth in kbit/s
    /// * `sel_class` - Optional selection class (for clients)
    /// * `mesh_if` - Mesh interface name
    ///
    /// # Example
    ///
    /// ```
    /// use robin::GwMode;
    /// client.set_gw_mode(GwMode::Server, Some(50000), Some(10000), None, "bat0").await?;
    /// ```
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

    /// Retrieves the global translation table entries.
    ///
    /// # Example
    ///
    /// ```
    /// let tg = client.transglobal("bat0").await?;
    /// for entry in tg {
    ///     println!("Client: {}", entry.client);
    /// }
    /// ```
    pub async fn transglobal(
        &self,
        mesh_if: &str,
    ) -> Result<Vec<model::TransglobalEntry>, RobinError> {
        commands::get_transglobal(mesh_if).await
    }

    /// Retrieves the local translation table entries.
    ///
    /// # Example
    ///
    /// ```
    /// let tl = client.translocal("bat0").await?;
    /// for entry in tl {
    ///     println!("Client: {}", entry.client);
    /// }
    /// ```
    pub async fn translocal(
        &self,
        mesh_if: &str,
    ) -> Result<Vec<model::TranslocalEntry>, RobinError> {
        commands::get_translocal(mesh_if).await
    }

    /// Retrieves the list of neighbors.
    ///
    /// # Example
    ///
    /// ```
    /// let neighbors = client.neighbors("bat0").await?;
    /// for n in neighbors {
    ///     println!("Neighbor: {}", n.neigh);
    /// }
    /// ```
    pub async fn neighbors(&self, mesh_if: &str) -> Result<Vec<model::Neighbor>, RobinError> {
        commands::get_neighbors(mesh_if).await
    }

    /// Retrieves the list of physical interfaces attached to the mesh.
    ///
    /// # Example
    ///
    /// ```
    /// let interfaces = client.get_interface("bat0").await?;
    /// for iface in interfaces {
    ///     println!("{}: {}", iface.ifname, iface.active);
    /// }
    /// ```
    pub async fn get_interface(&self, mesh_if: &str) -> Result<Vec<model::Interface>, RobinError> {
        commands::get_interfaces(mesh_if).await
    }

    /// Adds or removes a physical interface from the mesh.
    ///
    /// # Arguments
    /// * `iface` - Physical interface name
    /// * `mesh_if` - Some(mesh_if) to add, None to remove
    ///
    /// # Example
    ///
    /// ```
    /// client.set_interface("wlan1", Some("bat0")).await?;
    /// client.set_interface("wlan1", None).await?;
    /// ```
    pub async fn set_interface(
        &self,
        iface: &str,
        mesh_if: Option<&str>,
    ) -> Result<(), RobinError> {
        commands::set_interface(iface, mesh_if).await
    }

    /// Creates a new BATMAN-adv mesh interface with an optional routing algorithm.
    ///
    /// # Example
    ///
    /// ```
    /// client.create_interface("bat0", Some("BATMAN_V")).await?;
    /// client.create_interface("bat1", None).await?;
    /// ```
    pub async fn create_interface(
        &self,
        mesh_if: &str,
        routing_algo: Option<&str>,
    ) -> Result<(), RobinError> {
        commands::create_interface(mesh_if, routing_algo).await
    }

    /// Destroys a BATMAN-adv mesh interface.
    ///
    /// # Example
    ///
    /// ```
    /// client.destroy_interface("bat0").await?;
    /// ```
    pub async fn destroy_interface(&self, mesh_if: &str) -> Result<(), RobinError> {
        commands::destroy_interface(mesh_if).await
    }

    /// Counts the number of physical interfaces attached to the mesh.
    ///
    /// # Example
    ///
    /// ```
    /// let count = client.count_interfaces("bat0").await?;
    /// println!("Attached interfaces: {}", count);
    /// ```
    pub async fn count_interfaces(&self, mesh_if: &str) -> Result<u32, RobinError> {
        commands::count_interfaces(mesh_if).await
    }

    /// Checks whether packet aggregation is enabled on a BATMAN-adv mesh interface.
    ///
    /// Packet aggregation combines multiple packets into one to reduce overhead
    /// and improve throughput.
    ///
    /// # Arguments
    /// * `mesh_if` - The name of the mesh interface (e.g., `"bat0"`).
    ///
    /// # Example
    ///
    /// ```no_run
    /// let enabled = client.get_aggregation("bat0").await?;
    /// println!("Aggregation enabled? {}", enabled);
    /// ```
    pub async fn get_aggregation(&self, mesh_if: &str) -> Result<bool, RobinError> {
        commands::get_aggregation(mesh_if).await
    }

    /// Enables or disables packet aggregation on a mesh interface.
    ///
    /// # Arguments
    /// * `mesh_if` - The mesh interface name.
    /// * `val` - `true` to enable aggregation, `false` to disable.
    ///
    /// # Example
    ///
    /// ```no_run
    /// // Enable aggregation
    /// client.set_aggregation("bat0", true).await?;
    ///
    /// // Disable aggregation
    /// client.set_aggregation("bat0", false).await?;
    /// ```
    pub async fn set_aggregation(&self, mesh_if: &str, val: bool) -> Result<(), RobinError> {
        commands::set_aggregation(mesh_if, val).await
    }

    /// Checks whether AP isolation is enabled on a mesh interface.
    ///
    /// AP isolation prevents clients on the same Wi-Fi network from communicating
    /// directly, improving security in multi-client networks.
    ///
    /// # Arguments
    /// * `mesh_if` - The mesh interface name.
    ///
    /// # Example
    ///
    /// ```no_run
    /// let isolated = client.get_ap_isolation("bat0").await?;
    /// println!("AP isolation enabled? {}", isolated);
    /// ```
    pub async fn get_ap_isolation(&self, mesh_if: &str) -> Result<bool, RobinError> {
        commands::get_ap_isolation(mesh_if).await
    }

    /// Enables or disables AP isolation on a mesh interface.
    ///
    /// # Arguments
    /// * `mesh_if` - Mesh interface name.
    /// * `val` - `true` to enable isolation, `false` to disable.
    ///
    /// # Example
    ///
    /// ```no_run
    /// client.set_ap_isolation("bat0", true).await?; // enable
    /// client.set_ap_isolation("bat0", false).await?; // disable
    /// ```
    pub async fn set_ap_isolation(&self, mesh_if: &str, val: bool) -> Result<(), RobinError> {
        commands::set_ap_isolation(mesh_if, val).await
    }

    /// Checks whether bridge loop avoidance is enabled.
    ///
    /// Bridge loop avoidance prevents loops when multiple interfaces connect
    /// the same network segment.
    ///
    /// # Arguments
    /// * `mesh_if` - Mesh interface name.
    ///
    /// # Example
    ///
    /// ```no_run
    /// let enabled = client.get_bridge_loop_avoidance("bat0").await?;
    /// println!("Bridge loop avoidance: {}", enabled);
    /// ```
    pub async fn get_bridge_loop_avoidance(&self, mesh_if: &str) -> Result<bool, RobinError> {
        commands::get_bridge_loop_avoidance(mesh_if).await
    }

    /// Enables or disables bridge loop avoidance.
    ///
    /// # Arguments
    /// * `mesh_if` - Mesh interface name.
    /// * `val` - `true` to enable, `false` to disable.
    ///
    /// # Example
    ///
    /// ```no_run
    /// client.set_bridge_loop_avoidance("bat0", true).await?; // enable
    /// client.set_bridge_loop_avoidance("bat0", false).await?; // disable
    /// ```
    pub async fn set_bridge_loop_avoidance(
        &self,
        mesh_if: &str,
        val: bool,
    ) -> Result<(), RobinError> {
        commands::set_bridge_loop_avoidance(mesh_if, val).await
    }

    /// Retrieves the system default routing algorithm for BATMAN-adv.
    ///
    /// # Example
    ///
    /// ```no_run
    /// let default_algo = client.get_default_routing_algo().await?;
    /// println!("Default routing algorithm: {}", default_algo);
    /// ```
    pub async fn get_default_routing_algo(&self) -> Result<String, RobinError> {
        commands::get_default_routing_algo().await
    }

    /// Retrieves all active routing algorithms currently in use along with
    /// their corresponding mesh interfaces.
    ///
    /// Returns a vector of tuples `(interface_name, routing_algo_name)`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// let active_algos = client.get_active_routing_algos().await?;
    /// for (iface, algo) in active_algos {
    ///     println!("Interface {} uses {}", iface, algo);
    /// }
    /// ```
    pub async fn get_active_routing_algos(&self) -> Result<Vec<(String, String)>, RobinError> {
        commands::get_active_routing_algos().await
    }

    /// Retrieves the list of all routing algorithms available on the system.
    ///
    /// # Example
    ///
    /// ```no_run
    /// let available = client.get_available_routing_algos().await?;
    /// println!("Available routing algorithms:");
    /// for algo in available {
    ///     println!(" * {}", algo);
    /// }
    /// ```
    pub async fn get_available_routing_algos(&self) -> Result<Vec<String>, RobinError> {
        commands::get_available_routing_algos().await
    }

    /// Sets the system default routing algorithm.
    ///
    /// # Example
    ///
    /// ```
    /// client.set_default_routing_algo("BATMAN_V").await?;
    /// ```
    pub async fn set_default_routing_algo(&self, algo: &str) -> Result<(), RobinError> {
        commands::set_default_routing_algo(algo).await
    }
}
