use crate::robin::Neighbor;

use clap::Command;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Attribute, Cell, CellAlignment, Color, ContentArrangement, Table};

pub fn cmd_neighbors() -> Command {
    Command::new("neighbors")
        .alias("n")
        .about("Display the neighbor table.")
        .long_about("Display the neighbor table.")
        .override_usage(
            "robctl [options] n\n\
             robctl [options] neighbors",
        )
        //.disable_help_flag(true)
        .disable_version_flag(true)
}

pub fn print_neighbors(entries: &[Neighbor], algo_name: &str) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic);

    match algo_name {
        "BATMAN_IV" => {
            table.set_header(vec![
                Cell::new("IF"),
                Cell::new("Neighbor"),
                Cell::new("Last seen"),
            ]);
        }
        "BATMAN_V" => {
            table.set_header(vec![
                Cell::new("Neighbor"),
                Cell::new("Last seen"),
                Cell::new("Speed (Mbit/s)"),
                Cell::new("IF"),
            ]);
        }
        _ => return,
    }

    for n in entries {
        let last_seen_secs = n.last_seen_ms / 1000;
        let last_seen_msecs = n.last_seen_ms % 1000;

        let last_seen = format!("{}.{:03}s", last_seen_secs, last_seen_msecs);

        match algo_name {
            "BATMAN_IV" => {
                table.add_row(vec![
                    Cell::new(&n.outgoing_if),
                    Cell::new(n.neigh.to_string()),
                    Cell::new(last_seen).set_alignment(CellAlignment::Right),
                ]);
            }
            "BATMAN_V" => {
                let speed_cell = match n.throughput_kbps {
                    Some(kbits) => {
                        let mbit = kbits / 1000;
                        let rest = (kbits % 1000) / 100;

                        Cell::new(format!("{mbit}.{rest}"))
                            .set_alignment(CellAlignment::Right)
                            .fg(Color::Green)
                            .add_attribute(Attribute::Bold)
                    }
                    None => Cell::new("-"),
                };

                table.add_row(vec![
                    Cell::new(n.neigh.to_string()),
                    Cell::new(last_seen).set_alignment(CellAlignment::Right),
                    speed_cell,
                    Cell::new(&n.outgoing_if),
                ]);
            }
            _ => {}
        }
    }

    println!("{table}");
}
