use robin::Gateway;

use clap::Command;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Cell, CellAlignment, ContentArrangement, Table};

/// Creates the CLI command for displaying the list of gateways.
///
/// # Returns
/// - A `clap::Command` configured with:
///   - Name: `"gateways"`
///   - Alias: `"gwl"`
///   - Short and long description: `"Display the list of gateways."`
///   - Usage override: `robctl [options] gateways|gwl [options]`
///   - Version flag disabled
pub fn cmd_gateways() -> Command {
    Command::new("gateways")
        .alias("gwl")
        .about("Display the list of gateways.")
        .long_about("Display the list of gateways.")
        .override_usage("\trobctl [options] gateways|gwl [options]\n")
        .disable_version_flag(true)
}

/// Prints a formatted table of gateways to the console.
///
/// # Arguments
/// - `entries`: Slice of `Gateway` entries to display.
/// - `algo_name`: Name of the BATMAN algorithm used (`"BATMAN_IV"` or `"BATMAN_V"`).
///
/// # Behavior
/// - Configures the table headers differently depending on the algorithm:
///   - `"BATMAN_IV"`: Router, TQ, Next Hop, OutgoingIF, Bandwidth Down, Bandwidth Up
///   - `"BATMAN_V"`: Router, Throughput, Next Hop, OutgoingIF, Bandwidth Down, Bandwidth Up
/// - Highlights the best gateway with an asterisk (`*`) before the MAC address.
/// - Displays optional fields (`TQ`, `Throughput`, Bandwidth) with `0` if missing.
pub fn print_gwl(entries: &[Gateway], algo_name: &str) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic);

    match algo_name {
        "BATMAN_IV" => {
            table.set_header(vec![
                Cell::new("Router").set_alignment(CellAlignment::Center),
                Cell::new("TQ").set_alignment(CellAlignment::Center),
                Cell::new("Next Hop").set_alignment(CellAlignment::Center),
                Cell::new("OutgoingIF").set_alignment(CellAlignment::Center),
                Cell::new("Bandwidth Down (Mbit/s)").set_alignment(CellAlignment::Center),
                Cell::new("Bandwidth Up (Mbit/s)").set_alignment(CellAlignment::Center),
            ]);
        }
        "BATMAN_V" => {
            table.set_header(vec![
                Cell::new("Router").set_alignment(CellAlignment::Center),
                Cell::new("Throughput").set_alignment(CellAlignment::Center),
                Cell::new("Next Hop").set_alignment(CellAlignment::Center),
                Cell::new("OutgoingIF").set_alignment(CellAlignment::Center),
                Cell::new("Bandwidth Down (Mbit/s)").set_alignment(CellAlignment::Center),
                Cell::new("Bandwidth Up (Mbit/s)").set_alignment(CellAlignment::Center),
            ]);
        }
        _ => return,
    }

    for g in entries {
        let router_text = if g.is_best {
            format!("* {}", g.mac_addr)
        } else {
            g.mac_addr.to_string()
        };
        let router_cell = Cell::new(router_text);
        let next_hop_cell = Cell::new(g.router.to_string());

        match algo_name {
            "BATMAN_IV" => {
                table.add_row(vec![
                    router_cell.set_alignment(CellAlignment::Right),
                    Cell::new(g.tq.unwrap_or(0)),
                    next_hop_cell,
                    Cell::new(&g.outgoing_if),
                    Cell::new(g.bandwidth_down.unwrap_or(0)),
                    Cell::new(g.bandwidth_up.unwrap_or(0)),
                ]);
            }
            "BATMAN_V" => {
                table.add_row(vec![
                    router_cell.set_alignment(CellAlignment::Right),
                    Cell::new(g.throughput.unwrap_or(0)),
                    next_hop_cell,
                    Cell::new(&g.outgoing_if),
                    Cell::new(g.bandwidth_down.unwrap_or(0)),
                    Cell::new(g.bandwidth_up.unwrap_or(0)),
                ]);
            }
            _ => {}
        }
    }

    println!("{table}");
}
