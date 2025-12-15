use crate::robin::TranslocalEntry;

use clap::Command;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Cell, CellAlignment, ContentArrangement, Table};

pub fn cmd_translocal() -> Command {
    Command::new("translocal")
        .alias("tl")
        .about("Display local translation table.")
        .long_about("Display local translation table.")
        .override_usage(
            "robctl [options] tl\n\
             robctl [options] translocal",
        )
        //.disable_help_flag(true)
        .disable_version_flag(true)
}

pub fn print_translocal(entries: &[TranslocalEntry]) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic);

    table.set_header(vec![
        Cell::new("Client"),
        Cell::new("VID"),
        Cell::new("Flags"),
        Cell::new("Last seen"),
        Cell::new("CRC32"),
    ]);

    for e in entries {
        let r = if e.flags & 0x01 != 0 { 'R' } else { '.' };
        let p = if e.flags & 0x02 != 0 { 'P' } else { '.' };
        let n = if e.flags & 0x04 != 0 { 'N' } else { '.' };
        let x = if e.flags & 0x08 != 0 { 'X' } else { '.' };
        let w = if e.flags & 0x10 != 0 { 'W' } else { '.' };
        let i = if e.flags & 0x20 != 0 { 'I' } else { '.' };

        let client_cell = Cell::new(e.client.to_string());

        table.add_row(vec![
            client_cell,
            Cell::new(e.vid).set_alignment(CellAlignment::Right),
            Cell::new(format!("[{}{}{}{}{}{}]", r, p, n, x, w, i)),
            Cell::new(format!("{}.{:03}", e.last_seen_secs, e.last_seen_msecs))
                .set_alignment(CellAlignment::Right),
            Cell::new(format!("0x{:08x}", e.crc32)).set_alignment(CellAlignment::Right),
        ]);
    }

    println!("{table}");
}
