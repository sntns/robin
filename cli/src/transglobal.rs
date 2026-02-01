use crate::utils::print_vid;
use robin::TransglobalEntry;
use robin::model::ClientFlags;

use clap::Command;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Cell, CellAlignment, ContentArrangement, Table};

/// Creates the CLI command for displaying the global translation table.
///
/// # Returns
/// - A `clap::Command` configured with:
///   - Name: `"transglobal"`
///   - Alias: `"tg"`
///   - Short and long description: `"Display global translation table."`
///   - Usage override:
///       ```text
///       robctl [options] transglobal|tg [options]
///       ```
///   - Version flag disabled
pub fn cmd_transglobal() -> Command {
    Command::new("transglobal")
        .alias("tg")
        .about("Display global translation table.")
        .long_about("Display global translation table.")
        .override_usage("\trobctl [options] transglobal|tg [options]\n")
        .disable_version_flag(true)
}

/// Pretty-prints a list of `TransglobalEntry` into a table.
///
/// # Arguments
/// - `entries`: Slice of `TransglobalEntry` to display
///
/// # Table columns
/// - `Client`: MAC address of the client, with `*` prefix if it is the best entry
/// - `VID`: VLAN ID
/// - `Flags`: Concatenation of client flags:
///     - `R` = ROAM, `W` = WIFI, `I` = ISOLA, `T` = TEMP; `.` if not set
/// - `Last TTVN`: Last translation table version number seen for this entry
/// - `Originator`: MAC address of the originator node
/// - `TTVN`: Current translation table version number for this entry
/// - `CRC32`: CRC32 checksum in hexadecimal
pub fn print_transglobal(entries: &[TransglobalEntry]) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic);

    table.set_header(vec![
        Cell::new("Client").set_alignment(CellAlignment::Center),
        Cell::new("VID").set_alignment(CellAlignment::Center),
        Cell::new("Flags").set_alignment(CellAlignment::Center),
        Cell::new("Last TTVN").set_alignment(CellAlignment::Center),
        Cell::new("Originator").set_alignment(CellAlignment::Center),
        Cell::new("TTVN").set_alignment(CellAlignment::Center),
        Cell::new("CRC32").set_alignment(CellAlignment::Center),
    ]);

    for e in entries {
        let r = if e.flags.contains(ClientFlags::ROAM) {
            'R'
        } else {
            '.'
        };
        let w = if e.flags.contains(ClientFlags::WIFI) {
            'W'
        } else {
            '.'
        };
        let i = if e.flags.contains(ClientFlags::ISOLA) {
            'I'
        } else {
            '.'
        };
        let t = if e.flags.contains(ClientFlags::TEMP) {
            'T'
        } else {
            '.'
        };

        let client_text = if e.is_best {
            format!("* {}", e.client)
        } else {
            e.client.to_string()
        };
        let client_cell = Cell::new(client_text);
        let orig_cell = Cell::new(e.orig.to_string());

        table.add_row(vec![
            client_cell.set_alignment(CellAlignment::Right),
            Cell::new(print_vid(e.vid)),
            Cell::new(format!("[{}{}{}{}]", r, w, i, t)),
            Cell::new(e.ttvn),
            orig_cell,
            Cell::new(e.last_ttvn),
            Cell::new(format!("0x{:08x}", e.crc32)),
        ]);
    }

    println!("{table}");
}
