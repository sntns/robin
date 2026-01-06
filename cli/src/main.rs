use robin;

mod app;
mod gateways;
mod gw_mode;
mod interface;
mod neighbors;
mod originators;
mod transglobal;
mod translocal;
mod utils;

#[tokio::main]
async fn main() {
    let client = robin::RobinClient::new();
    let matches = app::build_cli().get_matches();
    let mesh_if = matches
        .get_one::<String>("meshif")
        .map(String::as_str)
        .unwrap_or("bat0");

    let algo_name = client.algo_name(mesh_if).await.unwrap();
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
            let interfaces: Vec<&str> = match sub_m.get_many::<String>("interfaces") {
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
                "destroy" => {
                    if !interfaces.is_empty() {
                        eprintln!("Error - extra parameter after '{}'", action);
                        return;
                    }
                    /*client
                    .destroy_mesh_interface(mesh_if)
                    .await
                    .unwrap();*/
                    return;
                }
                "create" => {
                    // create params parsing would go here (routing_algo)
                    /*client
                    .create_mesh_interface(mesh_if, parsing_param)
                    .await
                    .unwrap();*/
                    return;
                }
                "add" | "del" => {
                    if interfaces.is_empty() {
                        eprintln!("Error - missing interface name(s) after '{}'", action);
                        return;
                    }

                    let exists = client.if_nametoindex(mesh_if).await.unwrap_or(0);
                    if !manual && exists == 0 && action == "add" {
                        /*
                            client
                                .create_interface(mesh_if, None)
                                .await
                                .unwrap();
                        }*/
                        client.if_nametoindex(mesh_if).await.unwrap();
                    }

                    let pre_count = client.count_interfaces(mesh_if).await.unwrap();

                    for iface in &interfaces {
                        match action {
                            "add" => {
                                client.set_interface(iface, Some(mesh_if)).await.unwrap();
                            }
                            "del" => {
                                client.set_interface(iface, None).await.unwrap();
                            }
                            _ => unreachable!(),
                        }
                    }

                    if !manual && action == "del" {
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
        _ => unreachable!("Subcommand required"),
    }
}
