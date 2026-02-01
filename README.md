# batman-robin

[![crates.io](https://img.shields.io/crates/v/batman-robin.svg)](https://crates.io/crates/batman-robin)
[![docs.rs](https://img.shields.io/docsrs/batman-robin)](https://docs.rs/batman-robin)
[![CI](https://github.com/sntns/robin/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/sntns/robin/actions/workflows/ci.yml)
[![Coverage Status](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/someuser/coverage-badge.json)](https://github.com/sntns/robin)

`batman-robin` is a Rust library and CLI tool to interact with the **BATMAN-adv** kernel module, providing easy access to mesh network interfaces, neighbors, gateways, and routing algorithms.

---

## What are BATMAN and Robin?

**BATMAN-adv** (Better Approach To Mobile Ad-hoc Networking - advanced) is a Linux kernel module that implements a Layer 2 mesh networking protocol. Operating entirely within the kernel, BATMAN-adv routes Ethernet frames rather than IP packets, making all participating nodes appear as if they're on the same local area network (LAN). This protocol-agnostic approach enables transparent support for higher layer protocols like IPv4, IPv6, DHCP, and more across multiple physical media including WiFi, Ethernet, and VPN connections.

Key features of BATMAN-adv include:
- **Decentralized mesh networking**: No central controller required; each node autonomously manages routing
- **Layer 2 operation**: Routes Ethernet frames for protocol independence at higher layers
- **Multi-media support**: Works with WiFi, Ethernet, VPNs, and any Layer 2 interface
- **Dynamic topology handling**: Adapts to mobile and changing network environments
- **Minimal overhead**: Efficient in-kernel implementation

**Robin** is the companion tool that makes working with BATMAN-adv straightforward. Named after Batman's sidekick, `batman-robin` provides both a Rust library (for programmatic access) and a command-line interface (`robctl`) to manage and monitor BATMAN-adv mesh networks. With Robin, you can easily query mesh status, configure routing parameters, manage network interfaces, and monitor mesh topology—all without needing to work directly with low-level netlink APIs or kernel interfaces.

For more information about BATMAN-adv:
- [Linux Kernel BATMAN-adv Documentation](https://docs.kernel.org/networking/batman-adv.html)
- [BATMAN-adv Wiki and Guides](https://www.open-mesh.org/doc/batman-adv/Wiki.html)
- [BATMAN-adv GitHub Repository](https://github.com/open-mesh-mirror/batman-adv)

---

## Features

- Query and manage BATMAN-adv mesh interfaces
- Retrieve neighbors, originators, gateways, and translation tables
- Enable/disable aggregation, AP isolation, and bridge loop avoidance
- Inspect and set routing algorithms
- Provides both a Rust API and a command-line interface (`robctl`)

---

## Prerequisites

Before using Robin, you need to have the BATMAN-adv kernel module installed and loaded on your Linux system:

```bash
# Check if batman-adv is available
modinfo batman-adv

# Load the module
sudo modprobe batman-adv
```

For installation instructions and system requirements, see the [BATMAN-adv installation guide](https://www.open-mesh.org/doc/batman-adv/Wiki.html).

---

## Installation

Add `batman-robin` as a dependency in your `Cargo.toml`:

```toml
[dependencies]
batman-robin = { git = "https://github.com/sntns/robin.git" }
tokio = { version = "1", features = ["full"] }
```

Build the CLI:

```bash
cargo install --path .
```

This will install the `robctl` CLI tool.

---

## Rust API Usage

```rust
use batman_robin::{RobinClient, GwMode};
use tokio;

#[tokio::main]
async fn main() -> Result<(), batman_robin::RobinError> {
    let client = RobinClient::new();

    // Get default mesh interface
    let mesh_if = "bat0";

    // List neighbors
    let neighbors = client.neighbors(mesh_if).await?;
    for n in neighbors {
        println!("{:?}", n);
    }

    // Enable packet aggregation
    client.set_aggregation(mesh_if, true).await?;

    // Get gateway mode
    let gw_info = client.get_gw_mode(mesh_if).await?;
    println!("Gateway mode: {{:?}}", gw_info.mode);

    // Set default routing algorithm
    client.set_default_routing_algo("BATMAN_V").await?;

    Ok(())
}
```

### API Highlights

- **Interface Management**
  - `get_interface`, `set_interface`, `create_interface`, `destroy_interface`, `count_interfaces`
- **Mesh Settings**
  - `get_aggregation`, `set_aggregation`
  - `get_ap_isolation`, `set_ap_isolation`
  - `get_bridge_loop_avoidance`, `set_bridge_loop_avoidance`
- **Routing**
  - `get_default_routing_algo`, `get_active_routing_algos`, `get_available_routing_algos`, `set_default_routing_algo`
- **Gateway**
  - `get_gw_mode`, `set_gw_mode`
- **Network Tables**
  - `neighbors`, `originators`, `translocal`, `transglobal`, `gateways`

---

## CLI Usage

```bash
robctl --meshif bat0 neighbors
robctl --meshif bat0 gateways
robctl --meshif bat0 gw_mode
robctl --meshif bat0 originators
robctl --meshif bat0 translocal
robctl --meshif bat0 transglobal
robctl --meshif bat0 interface
robctl --meshif bat0 aggregation
robctl --meshif bat0 ap_isolation
robctl --meshif bat0 bridge_loop_avoidance
robctl --meshif bat0 routing_algo
```

### Examples

- **Check neighbors**

```bash
robctl -m bat0 neighbors
```

- **Enable aggregation**

```bash
robctl -m bat0 aggregation 1
```

- **Set gateway mode to server**

```bash
robctl -m bat0 gw_mode server 1000/500
```

- **Create a mesh interface with BATMAN_V**

```bash
robctl -m bat0 interface create ra BATMAN_V
```

- **Display the default routing algorithm**

```bash
robctl -m bat0 routing_algo
```

---

## Testing

To run tests:

```bash
cargo test --all
```

To check code formatting and lint:

```bash
cargo fmt -- --check
cargo clippy -- -D warnings
```

To check coverage (requires tarpaulin):

```bash
cargo install cargo-tarpaulin
cargo tarpaulin --out Xml
```

---

## Contributing

Contributions are welcome! You can:

- Open issues for bugs or feature requests
- Submit pull requests for improvements
- Add more examples and documentation

Please make sure to run `cargo fmt` and `cargo clippy` before submitting PRs.

---

## Learn More

### BATMAN-adv Resources

- **[Official BATMAN-adv Documentation](https://docs.kernel.org/networking/batman-adv.html)** - Linux kernel documentation for the batman-adv module
- **[BATMAN-adv Wiki](https://www.open-mesh.org/doc/batman-adv/Wiki.html)** - Comprehensive guides, tutorials, and configuration examples
- **[open-mesh.org](https://www.open-mesh.org/)** - The B.A.T.M.A.N. advanced project homepage
- **[BATMAN-adv GitHub](https://github.com/open-mesh-mirror/batman-adv)** - Source code and development repository

### Related Tools

- **[batctl](https://www.open-mesh.org/projects/batman-adv/wiki/Tweaking)** - The official BATMAN-adv control and management tool (C implementation)

---

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
