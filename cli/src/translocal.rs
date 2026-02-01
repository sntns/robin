use crate::utils::print_vid;
use robin::TranslocalEntry;
use robin::model::ClientFlags;

use clap::Command;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Cell, CellAlignment, ContentArrangement, Table};

/// Creates the CLI command for displaying the local translation table.
///
/// # Returns
/// - A `clap::Command` configured with:
///   - Name: `"translocal"`
///   - Alias: `"tl"`
///   - Short and long description: `"Display local translation table."`
///   - Usage override:
///       ```text
///       robctl [options] translocal|tl [options]
///       ```
///   - Version flag disabled
pub fn cmd_translocal() -> Command {
    Command::new("translocal")
        .alias("tl")
        .about("Display local translation table.")
        .long_about("Display local translation table.")
        .override_usage("\trobctl [options] translocal|tl [options]\n")
        .disable_version_flag(true)
}

/// Pretty-prints a list of `TranslocalEntry` into a table.
///
/// # Arguments
/// - `entries`: Slice of `TranslocalEntry` to display
///
/// # Table columns
/// - `Client`: MAC address of the client
/// - `VID`: VLAN ID
/// - `Flags`: Concatenation of client flags:
///     - `R` = ROAM, `P` = NOPURGE, `N` = NEW, `X` = PENDING,
///       `W` = WIFI, `I` = ISOLA; `.` if flag not set
/// - `Last seen`: Time since last seen, in seconds.milliseconds
/// - `CRC32`: CRC32 checksum in hexadecimal
pub fn print_translocal(entries: &[TranslocalEntry]) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic);

    table.set_header(vec![
        Cell::new("Client").set_alignment(CellAlignment::Center),
        Cell::new("VID").set_alignment(CellAlignment::Center),
        Cell::new("Flags").set_alignment(CellAlignment::Center),
        Cell::new("Last seen").set_alignment(CellAlignment::Center),
        Cell::new("CRC32").set_alignment(CellAlignment::Center),
    ]);

    for e in entries {
        let r = if e.flags.contains(ClientFlags::ROAM) {
            'R'
        } else {
            '.'
        };
        let p = if e.flags.contains(ClientFlags::NOPURGE) {
            'P'
        } else {
            '.'
        };
        let n = if e.flags.contains(ClientFlags::NEW) {
            'N'
        } else {
            '.'
        };
        let x = if e.flags.contains(ClientFlags::PENDING) {
            'X'
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

        let client_cell = Cell::new(e.client.to_string());

        table.add_row(vec![
            client_cell,
            Cell::new(print_vid(e.vid)),
            Cell::new(format!("[{}{}{}{}{}{}]", r, p, n, x, w, i)),
            Cell::new(format!("{}.{:03}", e.last_seen_secs, e.last_seen_msecs)),
            Cell::new(format!("0x{:08x}", e.crc32)),
        ]);
    }

    println!("{table}");
}
