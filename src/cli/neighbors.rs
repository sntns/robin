use crate::Neighbor;

use clap::Command;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Cell, CellAlignment, ContentArrangement, Table};
use macaddr::MacAddr6;
use std::collections::HashMap;

/// Creates the CLI command for displaying the neighbor table.
///
/// # Returns
/// - A `clap::Command` configured with:
///   - Name: `"neighbors"`
///   - Alias: `"n"`
///   - Short and long description: `"Display the neighbor table."`
///   - Usage override:
///       ```text
///       robctl [options] neighbors|n [options]
///       ```
///   - Version flag disabled
pub fn cmd_neighbors() -> Command {
    Command::new("neighbors")
        .alias("n")
        .about("Display the neighbor table.")
        .long_about("Display the neighbor table.")
        .override_usage("\trobctl [options] neighbors|n [options]\n")
        .disable_version_flag(true)
}

/// Deduplicates neighbors based on `(neighbor MAC, outgoing interface)`.
///
/// # Arguments
/// - `neighbors`: A vector of `Neighbor` entries.
///
/// # Behavior
/// - If multiple entries exist for the same `(MAC, interface)`, keeps the one with the **latest `last_seen_ms`**.
/// - Returns a deduplicated `Vec<Neighbor>`.
pub fn dedup_neighbors(neighbors: Vec<Neighbor>) -> Vec<Neighbor> {
    let mut map: HashMap<(MacAddr6, String), Neighbor> = HashMap::new();

    for n in neighbors {
        let key = (n.neigh, n.outgoing_if.clone());
        match map.get(&key) {
            Some(existing) => {
                if n.last_seen_ms < existing.last_seen_ms {
                    map.insert(key, n);
                }
            }
            None => {
                map.insert(key, n);
            }
        }
    }

    map.into_values().collect()
}

/// Prints a neighbor table in a human-readable format.
///
/// # Arguments
/// - `entries`: Slice of `Neighbor` entries.
/// - `algo_name`: Name of the routing algorithm (BATMAN_IV or BATMAN_V).
///
/// # Behavior
/// - For BATMAN_IV:
///     - Columns: `"IF"`, `"Neighbor"`, `"Last seen"`
/// - For BATMAN_V:
///     - Columns: `"Neighbor"`, `"Last seen"`, `"Speed (Mbit/s)"`, `"IF"`
/// - Deduplicates entries before printing.
/// - `last_seen_ms` is formatted as seconds with milliseconds precision.
pub fn print_neighbors(entries: &[Neighbor], algo_name: &str) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic);

    match algo_name {
        "BATMAN_IV" => {
            table.set_header(vec![
                Cell::new("IF").set_alignment(CellAlignment::Center),
                Cell::new("Neighbor").set_alignment(CellAlignment::Center),
                Cell::new("Last seen").set_alignment(CellAlignment::Center),
            ]);
        }
        "BATMAN_V" => {
            table.set_header(vec![
                Cell::new("Neighbor").set_alignment(CellAlignment::Center),
                Cell::new("Last seen").set_alignment(CellAlignment::Center),
                Cell::new("Speed (Mbit/s)").set_alignment(CellAlignment::Center),
                Cell::new("IF").set_alignment(CellAlignment::Center),
            ]);
        }
        _ => return,
    }

    let dedup_entries = dedup_neighbors(entries.to_vec());
    for n in dedup_entries {
        let last_seen_secs = n.last_seen_ms / 1000;
        let last_seen_msecs = n.last_seen_ms % 1000;
        let last_seen = format!("{}.{:03}s", last_seen_secs, last_seen_msecs);

        match algo_name {
            "BATMAN_IV" => {
                table.add_row(vec![
                    Cell::new(&n.outgoing_if),
                    Cell::new(n.neigh.to_string()),
                    Cell::new(last_seen),
                ]);
            }
            "BATMAN_V" => {
                let speed_cell = match n.throughput_kbps {
                    Some(kbits) => {
                        let mbit = kbits / 1000;
                        let rest = (kbits % 1000) / 100;

                        Cell::new(format!("{mbit}.{rest}"))
                    }
                    None => Cell::new("-"),
                };

                table.add_row(vec![
                    Cell::new(n.neigh.to_string()),
                    Cell::new(last_seen),
                    speed_cell,
                    Cell::new(&n.outgoing_if),
                ]);
            }
            _ => {}
        }
    }

    println!("{table}");
}
