use crate::robin::TransglobalEntry;
use crate::robin::model::ClientFlags;
use crate::utils::print_vid;

use clap::Command;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Attribute, Cell, CellAlignment, Color, ContentArrangement, Table};

pub fn cmd_transglobal() -> Command {
    Command::new("transglobal")
        .alias("tg")
        .about("Display global translation table.")
        .long_about("Display global translation table.")
        .override_usage(
            "robctl [options] tg\n\
             robctl [options] transglobal",
        )
        //.disable_help_flag(true)
        .disable_version_flag(true)
}

pub fn print_transglobal(entries: &[TransglobalEntry]) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic);

    table.set_header(vec![
        Cell::new("Client"),
        Cell::new("VID"),
        Cell::new("Flags"),
        Cell::new("Last TTVN"),
        Cell::new("Originator"),
        Cell::new("TTVN"),
        Cell::new("CRC32"),
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

        let mut client_cell = Cell::new(e.client.to_string());
        let mut orig_cell = Cell::new(e.orig.to_string());
        if e.is_best {
            client_cell = client_cell.fg(Color::Green).add_attribute(Attribute::Bold);

            orig_cell = orig_cell.fg(Color::Green).add_attribute(Attribute::Bold);
        }

        table.add_row(vec![
            client_cell,
            Cell::new(print_vid(e.vid)).set_alignment(CellAlignment::Right),
            Cell::new(format!("[{}{}{}{}]", r, w, i, t)),
            Cell::new(e.ttvn).set_alignment(CellAlignment::Right),
            orig_cell,
            Cell::new(e.last_ttvn).set_alignment(CellAlignment::Right),
            Cell::new(format!("0x{:08x}", e.crc32)).set_alignment(CellAlignment::Right),
        ]);
    }

    println!("{table}");
}
