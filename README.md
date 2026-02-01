# robin

[![crates.io](https://img.shields.io/crates/v/robin.svg)](https://crates.io/crates/robin)
[![docs.rs](https://img.shields.io/docsrs/robin)](https://docs.rs/robin)
[![CI](https://github.com/sntns/robin/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/sntns/robin/actions/workflows/ci.yml)
[![Coverage Status](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/someuser/coverage-badge.json)](https://github.com/sntns/robin)

`robin` is a Rust library and CLI tool to interact with the **BATMAN-adv** kernel module, providing easy access to mesh network interfaces, neighbors, gateways, and routing algorithms.

---

## Features

- Query and manage BATMAN-adv mesh interfaces
- Retrieve neighbors, originators, gateways, and translation tables
- Enable/disable aggregation, AP isolation, and bridge loop avoidance
- Inspect and set routing algorithms
- Provides both a Rust API and a command-line interface (`robctl`)

---

## Installation

Add `robin` as a dependency in your `Cargo.toml`:

```toml
[dependencies]
robin = { git = "https://github.com/sntns/robin.git" }
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
use robin::{RobinClient, GwMode};
use tokio;

#[tokio::main]
async fn main() -> Result<(), robin::RobinError> {
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

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
