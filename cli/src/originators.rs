use crate::robin::Originator;

use clap::Command;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Attribute, Cell, CellAlignment, Color, ContentArrangement, Table};

pub fn cmd_originators() -> Command {
    Command::new("originators")
        .alias("o")
        .about("Display the originator table.")
        .long_about("Display the originator table.")
        .override_usage(
            "robctl [options] o\n\
             robctl [options] originators",
        )
        //.disable_help_flag(true)
        .disable_version_flag(true)
}

pub fn print_originators(entries: &[Originator], algo_name: &str) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic);

    match algo_name {
        "BATMAN_IV" => {
            table.set_header(vec![
                Cell::new("Originator"),
                Cell::new("Last seen"),
                Cell::new("TQ"),
                Cell::new("Next hop"),
                Cell::new("Outgoing IF"),
            ]);
        }
        "BATMAN_V" => {
            table.set_header(vec![
                Cell::new("Originator"),
                Cell::new("Last seen"),
                Cell::new("Throughput (Mbit/s)"),
                Cell::new("Next hop"),
                Cell::new("Outgoing IF"),
            ]);
        }
        _ => return,
    }

    for o in entries {
        let last_seen_secs = o.last_seen_ms / 1000;
        let last_seen_msecs = o.last_seen_ms % 1000;

        let last_seen = format!("{}.{:03}s", last_seen_secs, last_seen_msecs);

        let mut originator_cell = Cell::new(o.originator.to_string());
        let mut next_hop_cell = Cell::new(o.next_hop.to_string());
        if o.is_best {
            originator_cell = originator_cell
                .fg(Color::Green)
                .add_attribute(Attribute::Bold);

            next_hop_cell = next_hop_cell
                .fg(Color::Green)
                .add_attribute(Attribute::Bold);
        }

        match algo_name {
            "BATMAN_IV" => {
                let tq = o.tq.unwrap_or(0);

                table.add_row(vec![
                    originator_cell,
                    Cell::new(last_seen).set_alignment(CellAlignment::Right),
                    Cell::new(format!("{}/255", tq)).set_alignment(CellAlignment::Right),
                    next_hop_cell,
                    Cell::new(&o.outgoing_if),
                ]);
            }

            "BATMAN_V" => {
                let throughput_cell = match o.throughput {
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
                    originator_cell,
                    Cell::new(last_seen).set_alignment(CellAlignment::Right),
                    throughput_cell,
                    next_hop_cell,
                    Cell::new(&o.outgoing_if),
                ]);
            }
            _ => {}
        }
    }

    println!("{table}");
}
