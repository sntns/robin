use crate::robin::Gateway;

use clap::Command;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Attribute, Cell, CellAlignment, Color, ContentArrangement, Table};

pub fn cmd_gateways() -> Command {
    Command::new("gateways")
        .alias("gwl")
        .about("Display the list of gateways.")
        .long_about("Display the list of gateways.")
        .override_usage(
            "robctl [options] gwl\n\
            robctl [options] gateways",
        )
        //.disable_help_flag(true)
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
                Cell::new("Router"),
                Cell::new("TQ"),
                Cell::new("Next Hop"),
                Cell::new("OutgoingIF"),
                Cell::new("Bandwidth Down (Mbit/s)"),
                Cell::new("Bandwidth Up (Mbit/s)"),
            ]);
        }
        "BATMAN_V" => {
            table.set_header(vec![
                Cell::new("Router"),
                Cell::new("Throughput"),
                Cell::new("Next Hop"),
                Cell::new("OutgoingIF"),
                Cell::new("Bandwidth Down (Mbit/s)"),
                Cell::new("Bandwidth Up (Mbit/s)"),
            ]);
        }
        _ => return,
    }

    for g in entries {
        let mut router_cell = Cell::new(g.mac_addr.to_string());
        let mut next_hop_cell = Cell::new(g.router.to_string());
        if g.is_best {
            router_cell = router_cell.fg(Color::Green).add_attribute(Attribute::Bold);

            next_hop_cell = next_hop_cell
                .fg(Color::Green)
                .add_attribute(Attribute::Bold);
        }

        match algo_name {
            "BATMAN_IV" => {
                table.add_row(vec![
                    router_cell,
                    Cell::new(g.tq.unwrap_or(0)).set_alignment(CellAlignment::Right),
                    next_hop_cell,
                    Cell::new(&g.outgoing_if),
                    Cell::new(g.bandwidth_down.unwrap_or(0)).set_alignment(CellAlignment::Right),
                    Cell::new(g.bandwidth_up.unwrap_or(0)).set_alignment(CellAlignment::Right),
                ]);
            }
            "BATMAN_V" => {
                table.add_row(vec![
                    router_cell,
                    Cell::new(g.throughput.unwrap_or(0)).set_alignment(CellAlignment::Right),
                    next_hop_cell,
                    Cell::new(&g.outgoing_if),
                    Cell::new(g.bandwidth_down.unwrap_or(0)).set_alignment(CellAlignment::Right),
                    Cell::new(g.bandwidth_up.unwrap_or(0)).set_alignment(CellAlignment::Right),
                ]);
            }
            _ => {}
        }
    }

    println!("{table}");
}
