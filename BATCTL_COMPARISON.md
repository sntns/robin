# batctl Command Comparison

This document provides a comprehensive comparison between the official `batctl` tool and the `batman-robin` (Robin) implementation. It details which batctl commands are implemented in Robin and which are not yet available.

## Overview

Robin is a Rust reimplementation of the core `batctl` functionality, providing both a library API (`RobinClient`) and a command-line interface (`robctl`). While Robin covers the most commonly used batctl features for mesh network management, some advanced diagnostic and configuration options are not yet implemented.

## Implemented Features

The following batctl commands are fully implemented in Robin:

### Interface Management

| batctl Command | Robin API Method | robctl Command | Description |
|----------------|------------------|----------------|-------------|
| `interface [show]` | `get_interface()` | `robctl interface` | Display interfaces added to the mesh |
| `interface add <if>` | `set_interface()` | `robctl interface add <if>` | Add an interface to the mesh |
| `interface del <if>` | `set_interface()` | `robctl interface del <if>` | Remove an interface from the mesh |
| `interface create` | `create_interface()` | `robctl interface create` | Create a new mesh interface |
| `interface destroy` | `destroy_interface()` | `robctl interface destroy` | Destroy a mesh interface |

### Mesh Configuration Settings

| batctl Command | Robin API Method | robctl Command | Description |
|----------------|------------------|----------------|-------------|
| `aggregation [0\\|1]` | `get_aggregation()`, `set_aggregation()` | `robctl aggregation [0\\|1]` | Get/set packet aggregation |
| `ap_isolation [0\\|1]` | `get_ap_isolation()`, `set_ap_isolation()` | `robctl ap_isolation [0\\|1]` | Get/set AP isolation |
| `bridge_loop_avoidance [0\\|1]` | `get_bridge_loop_avoidance()`, `set_bridge_loop_avoidance()` | `robctl bridge_loop_avoidance [0\\|1]` | Get/set bridge loop avoidance |

### Routing Algorithm Management

| batctl Command | Robin API Method | robctl Command | Description |
|----------------|------------------|----------------|-------------|
| `routing_algo [show]` | `get_default_routing_algo()` | `robctl routing_algo` | Show default routing algorithm |
| `routing_algo [algo]` | `set_default_routing_algo()` | `robctl routing_algo <algo>` | Set default routing algorithm |
| N/A | `get_active_routing_algos()` | N/A | List currently active algorithms |
| N/A | `get_available_routing_algos()` | N/A | List available algorithms |

### Gateway Management

| batctl Command | Robin API Method | robctl Command | Description |
|----------------|------------------|----------------|-------------|
| `gw_mode [mode]` | `get_gw_mode()`, `set_gw_mode()` | `robctl gw_mode [mode]` | Get/set gateway mode (off/client/server) |
| `gateways` | `gateways()` | `robctl gateways` | List available gateways |

### Network Information Tables

| batctl Command | Robin API Method | robctl Command | Description |
|----------------|------------------|----------------|-------------|
| `neighbors` | `neighbors()` | `robctl neighbors` | List neighbor nodes |
| `originators` | `originators()` | `robctl originators` | Show originator table |
| `translocal` | `translocal()` | `robctl translocal` | Show local translation table |
| `transglobal` | `transglobal()` | `robctl transglobal` | Show global translation table |

### Utility Methods

| Robin API Method | Description |
|------------------|-------------|
| `if_nametoindex()` | Convert interface name to index |
| `if_indextoname()` | Convert interface index to name |
| `count_interfaces()` | Count mesh interfaces |

## Not Yet Implemented

The following batctl features are **not yet implemented** in Robin:

### Configuration Settings

| batctl Command | Description | Priority |
|----------------|-------------|----------|
| `orig_interval [ms]` | Set originator message interval | Medium |
| `distributed_arp_table [0\\|1]` | Enable/disable DAT (Distributed ARP Table) | Medium |
| `bonding [0\\|1]` | Enable/disable bonding mode | Medium |
| `fragmentation [0\\|1]` | Enable/disable fragmentation | Medium |
| `multicast_mode [0\\|1]` | Enable/disable optimized multicast forwarding | Medium |
| `network_coding [0\\|1]` | Enable/disable network coding | Low |
| `hop_penalty [penalty]` | Set hop penalty | Low |
| `isolation_mark [mark]` | Set isolation mark | Low |

### Diagnostic Tools

| batctl Command | Description | Priority |
|----------------|-------------|----------|
| `ping <MAC\\|host>` | Layer-2 batman ping | High |
| `traceroute <MAC\\|host>` | Layer-2 traceroute | High |
| `tcpdump` | Print batman-adv frames | Medium |
| `throughputmeter <MAC\\|host>` | Measure throughput to peer | Medium |
| `event [-t\\|-r]` | Show batman-adv kernel events | Medium |

### Debug Tables

| batctl Command | Description | Priority |
|----------------|-------------|----------|
| `dat_cache` | Show distributed ARP cache | Medium |
| `bla_claim` | Show bridge loop avoidance claims | Medium |
| `bla_backbone` | Show bridge loop avoidance backbone | Medium |
| `log` | Show kernel module log buffer | Low |
| `mcast_flags` | Show multicast flags | Low |

### Advanced Features

| batctl Command | Description | Priority |
|----------------|-------------|----------|
| `bisect_iv [options]` | Analyze OGM traffic from logs | Low |
| `statistics` | Show interface statistics | Medium |
| `hardif <interface>` | Specify hard interface | Low |
| `vlan` | VLAN-specific operations | Low |

## Implementation Status Summary

- ✅ **Implemented**: 24 core API methods covering essential mesh management
- ⏳ **Not Implemented**: ~25+ advanced batctl features
- 📊 **Coverage**: Approximately 50-60% of batctl's total functionality

## Priority Explanation

- **High**: Core diagnostic features that would significantly enhance Robin's utility
- **Medium**: Useful configuration and monitoring features
- **Low**: Advanced or rarely-used features

## Contributing

If you'd like to help implement any of the missing features, please:

1. Open an issue to discuss the implementation approach
2. Check the [BATMAN-adv documentation](https://docs.kernel.org/networking/batman-adv.html) for technical details
3. Follow the coding standards in `.github/copilot-instructions.md`
4. Ensure new features include appropriate tests and documentation

## References

- [batctl man page](https://manpages.debian.org/buster/batctl/batctl.8.en.html)
- [BATMAN-adv Wiki](https://www.open-mesh.org/doc/batman-adv/Wiki.html)
- [Using batctl Guide](https://www.open-mesh.org/projects/batman-adv/wiki/Using-batctl)
