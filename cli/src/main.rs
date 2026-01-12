use robin;

mod aggregation;
mod ap_isolation;
mod app;
mod bridge_loop_avoidance;
mod gateways;
mod gw_mode;
mod interface;
mod neighbors;
mod originators;
mod routing_algo;
mod transglobal;
mod translocal;
mod utils;

// TODO: do ap_isolation, bridge... and aggregation with netlink like in batctl
// TODO: add netlink part in routing_algo like in batctl in order to give available algorithms etc
// TODO: remove get_algo_name in commands/utils.rs after refactoring routing_algo

#[tokio::main]
async fn main() {
    let client = robin::RobinClient::new();
    let matches = app::build_cli().get_matches();
    let mesh_if = matches
        .get_one::<String>("meshif")
        .map(String::as_str)
        .unwrap_or("bat0");

    let algo_name = client.get_routing_algo().await.unwrap();
    if matches.get_flag("version") {
        println!(
            "robctl version: {} [{}]",
            env!("CARGO_PKG_VERSION"),
            algo_name
        );
        return;
    }

    match matches.subcommand() {
        Some(("neighbors", _)) => {
            let entries = client.neighbors(mesh_if).await.unwrap();
            neighbors::print_neighbors(&entries, algo_name.as_str());
        }
        Some(("gateways", _)) => {
            let entries = client.gateways(mesh_if).await.unwrap();
            gateways::print_gwl(&entries, algo_name.as_str());
        }
        Some(("gw_mode", sub_m)) => {
            let mode_str = sub_m.get_one::<String>("mode").map(String::as_str);
            let param_str = sub_m.get_one::<String>("param").map(String::as_str);

            if mode_str.is_none() {
                let entries = client.get_gw_mode(mesh_if).await.unwrap();
                gw_mode::print_gw(&entries);
                return;
            }

            let mode = match mode_str.unwrap() {
                "off" => robin::GwMode::Off,
                "client" => robin::GwMode::Client,
                "server" => robin::GwMode::Server,
                other => {
                    eprintln!("Invalid mode: {}", other);
                    return;
                }
            };

            let (down, up, sel_class) = if let Some(param) = param_str {
                gw_mode::parse_gw_param(mode, param).unwrap()
            } else {
                (None, None, None)
            };

            client
                .set_gw_mode(mode, down, up, sel_class, mesh_if)
                .await
                .unwrap();
        }
        Some(("originators", _)) => {
            let entries = client.originators(mesh_if).await.unwrap();
            originators::print_originators(&entries, algo_name.as_str());
        }
        Some(("translocal", _)) => {
            let entries = client.translocal(mesh_if).await.unwrap();
            translocal::print_translocal(&entries);
        }
        Some(("transglobal", _)) => {
            let entries = client.transglobal(mesh_if).await.unwrap();
            transglobal::print_transglobal(&entries);
        }
        Some(("interface", sub_m)) => {
            let manual = sub_m.get_flag("manual");
            let action = sub_m.get_one::<String>("action").map(String::as_str);
            let params: Vec<&str> = match sub_m.get_many::<String>("params") {
                Some(vals) => vals.map(String::as_str).collect(),
                None => Vec::new(),
            };

            if action.is_none() {
                let entries = client.get_interface(mesh_if).await.unwrap();
                interface::print_interfaces(&entries);
                return;
            }

            let action = action.unwrap();
            match action {
                "destroy" | "D" => {
                    if !params.is_empty() {
                        eprintln!("Error - extra parameter after '{}'", action);
                        return;
                    }
                    client.destroy_interface(mesh_if).await.unwrap();
                    return;
                }
                "create" | "c" => {
                    let routing_algo = match params.as_slice() {
                        [] => None,
                        ["ra", algo] => Some(*algo),
                        ["routing_algo", algo] => Some(*algo),
                        _ => {
                            eprintln!("Error - invalid parameters for create");
                            return;
                        }
                    };

                    client
                        .create_interface(mesh_if, routing_algo)
                        .await
                        .unwrap();
                    return;
                }
                "add" | "a" | "del" | "d" => {
                    if params.is_empty() {
                        eprintln!("Error - missing interface name(s) after '{}'", action);
                        return;
                    }

                    let exists = client.if_nametoindex(mesh_if).await.unwrap_or(0);
                    if !manual && exists == 0 && action.starts_with("a") {
                        /*
                            client
                                .create_interface(mesh_if, None)
                                .await
                                .unwrap();
                        }*/
                        client.if_nametoindex(mesh_if).await.unwrap();
                    }

                    let pre_count = client.count_interfaces(mesh_if).await.unwrap();

                    for iface in &params {
                        match action {
                            "add" | "a" => {
                                client.set_interface(iface, Some(mesh_if)).await.unwrap();
                            }
                            "del" | "d" => {
                                client.set_interface(iface, None).await.unwrap();
                            }
                            _ => unreachable!(),
                        }
                    }

                    if !manual && (action == "del" || action == "d") {
                        let cnt = client.count_interfaces(mesh_if).await.unwrap();

                        if cnt == 0 && pre_count > 0 {
                            println!(
                                "Warning: {} has no interfaces and can be destroyed with: robctl meshif {} interface destroy",
                                mesh_if, mesh_if
                            );
                        }
                    }
                }
                _ => {}
            }
        }
        Some(("aggregation", sub_m)) => {
            let val = sub_m.get_one::<u8>("value");
            if let Some(v) = val {
                client.set_aggregation(mesh_if, *v == 1).await.unwrap();
            } else {
                let enabled = client.get_aggregation(mesh_if).await.unwrap();
                println!("{}", enabled as u8);
            }
        }
        Some(("ap_isolation", sub_m)) => {
            let val = sub_m.get_one::<u8>("value");
            if let Some(v) = val {
                client.set_ap_isolation(mesh_if, *v == 1).await.unwrap();
            } else {
                let enabled = client.get_ap_isolation(mesh_if).await.unwrap();
                println!("{}", enabled as u8);
            }
        }
        Some(("bridge_loop_avoidance", sub_m)) => {
            let val = sub_m.get_one::<u8>("value");
            if let Some(v) = val {
                client
                    .set_bridge_loop_avoidance(mesh_if, *v == 1)
                    .await
                    .unwrap();
            } else {
                let enabled = client.get_bridge_loop_avoidance(mesh_if).await.unwrap();
                println!("{}", enabled as u8);
            }
        }
        Some(("routing_algo", sub_m)) => {
            if let Some(algo) = sub_m.get_one::<String>("algo") {
                client.set_routing_algo(algo).await.unwrap();
            } else {
                let algo = client.get_routing_algo().await.unwrap();
                println!("{}", algo);
            }
        }
        _ => unreachable!("Subcommand required"),
    }
}
