use robin::Originator;

use clap::Command;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Cell, CellAlignment, ContentArrangement, Table};

/// Creates the CLI command for displaying the originator table.
///
/// # Returns
/// - A `clap::Command` configured with:
///   - Name: `"originators"`
///   - Alias: `"o"`
///   - Short and long description: `"Display the originator table."`
///   - Usage override:
///       ```text
///       robctl [options] originators|o [options]
///       ```
///   - Version flag disabled
pub fn cmd_originators() -> Command {
    Command::new("originators")
        .alias("o")
        .about("Display the originator table.")
        .long_about("Display the originator table.")
        .override_usage("\trobctl [options] originators|o [options]\n")
        .disable_version_flag(true)
}

/// Prints a formatted originator table.
///
/// # Arguments
/// - `entries`: Slice of `Originator` entries.
/// - `algo_name`: Name of the routing algorithm (BATMAN_IV or BATMAN_V).
///
/// # Behavior
/// - For BATMAN_IV:
///     - Columns: `"Originator"`, `"Last seen"`, `"TQ"`, `"Next hop"`, `"Outgoing IF"`
///     - TQ is displayed as `value/255`
/// - For BATMAN_V:
///     - Columns: `"Originator"`, `"Last seen"`, `"Throughput (Mbit/s)"`, `"Next hop"`, `"Outgoing IF"`
///     - Throughput is converted from kbit/s to Mbit with one decimal place
/// - Marks best originators with a `*` prefix.
/// - `last_seen_ms` is formatted as seconds with milliseconds precision.
pub fn print_originators(entries: &[Originator], algo_name: &str) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic);

    match algo_name {
        "BATMAN_IV" => {
            table.set_header(vec![
                Cell::new("Originator").set_alignment(CellAlignment::Center),
                Cell::new("Last seen").set_alignment(CellAlignment::Center),
                Cell::new("TQ").set_alignment(CellAlignment::Center),
                Cell::new("Next hop").set_alignment(CellAlignment::Center),
                Cell::new("Outgoing IF").set_alignment(CellAlignment::Center),
            ]);
        }
        "BATMAN_V" => {
            table.set_header(vec![
                Cell::new("Originator").set_alignment(CellAlignment::Center),
                Cell::new("Last seen").set_alignment(CellAlignment::Center),
                Cell::new("Throughput (Mbit/s)").set_alignment(CellAlignment::Center),
                Cell::new("Next hop").set_alignment(CellAlignment::Center),
                Cell::new("Outgoing IF").set_alignment(CellAlignment::Center),
            ]);
        }
        _ => return,
    }

    for o in entries {
        let last_seen_secs = o.last_seen_ms / 1000;
        let last_seen_msecs = o.last_seen_ms % 1000;
        let last_seen = format!("{}.{:03}s", last_seen_secs, last_seen_msecs);

        let originator_text = if o.is_best {
            format!("* {}", o.originator)
        } else {
            o.originator.to_string()
        };
        let originator_cell = Cell::new(originator_text);
        let next_hop_cell = Cell::new(o.next_hop.to_string());

        match algo_name {
            "BATMAN_IV" => {
                let tq = o.tq.unwrap_or(0);

                table.add_row(vec![
                    originator_cell.set_alignment(CellAlignment::Right),
                    Cell::new(last_seen),
                    Cell::new(format!("{}/255", tq)),
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
                    }
                    None => Cell::new("-"),
                };

                table.add_row(vec![
                    originator_cell.set_alignment(CellAlignment::Right),
                    Cell::new(last_seen),
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
