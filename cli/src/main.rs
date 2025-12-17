use robin;

mod app;
mod gateways;
mod gw_mode;
mod neighbors;
mod originators;
mod transglobal;
mod translocal;
mod utils;

// TODO: fix neighbor command where everything is doubled

#[tokio::main]
async fn main() {
    let client = robin::RobinClient::new();
    let matches = app::build_cli().get_matches();
    let algo_name = client.algo_name().await.unwrap();

    match matches.subcommand() {
        Some(("neighbors", _)) => {
            let entries = client.neighbors().await.unwrap();
            neighbors::print_neighbors(&entries, algo_name.as_str());
        }
        Some(("gateways", _)) => {
            let entries = client.gateways().await.unwrap();
            gateways::print_gwl(&entries, algo_name.as_str());
        }
        Some(("gw_mode", sub_m)) => {
            let mode_str = sub_m.get_one::<String>("mode").map(String::as_str);
            let param_str = sub_m.get_one::<String>("param").map(String::as_str);

            if mode_str.is_none() {
                let entries = client.get_gw_mode().await.unwrap();
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

            client.set_gw_mode(mode, down, up, sel_class).await.unwrap();

            // TODO: Not sure
            let entries = client.get_gw_mode().await.unwrap();
            gw_mode::print_gw(&entries);
            //
        }
        Some(("originators", _)) => {
            let entries = client.originators().await.unwrap();
            originators::print_originators(&entries, algo_name.as_str());
        }
        Some(("translocal", _)) => {
            let entries = client.translocal().await.unwrap();
            translocal::print_translocal(&entries);
        }
        Some(("transglobal", _)) => {
            let entries = client.transglobal().await.unwrap();
            transglobal::print_transglobal(&entries);
        }
        _ => unreachable!("Subcommand required"),
    }
}
