use crate::robin::Gateway;

use clap::Command;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Cell, CellAlignment, ContentArrangement, Table};

pub fn cmd_gateways() -> Command {
    Command::new("gateways")
        .alias("gwl")
        .about("Display the list of gateways.")
        .long_about("Display the list of gateways.")
        .override_usage(
            "robctl [options] gwl\n\
            robctl [options] gateways",
        )
        .disable_version_flag(true)
}

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
