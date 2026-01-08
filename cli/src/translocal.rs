use crate::robin::TranslocalEntry;
use crate::robin::model::ClientFlags;
use crate::utils::print_vid;

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
        .disable_version_flag(true)
}

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
