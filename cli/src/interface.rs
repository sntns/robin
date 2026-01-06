use robin::Interface;

use clap::{Arg, ArgMatches, Command};

// TODO: add create and destroy and routing algo optional parameter for create
pub fn cmd_interfaces() -> Command {
    Command::new("interface")
        .alias("if")
        .about("Display or modify the batman-adv interface settings")
        .long_about(
            "Display or modify the batman-adv interface settings.\n\
             Usage:\n\
             \trobctl if                 # list interfaces\n\
             \trobctl if add iface1 ...  # add interfaces to mesh\n\
             \trobctl if del iface1 ...  # remove interfaces from mesh\n\
             \t-M disables automatic mesh creation/destruction",
        )
        .arg(
            Arg::new("manual")
                .short('M')
                .help("Disable automatic creation/destruction of batman-adv interface"),
        )
        .arg(
            Arg::new("action")
                .index(1)
                .value_parser(["add", "del"])
                .help("Action to perform: add or del"),
        )
        .arg(
            Arg::new("interfaces")
                .index(2)
                .num_args(1..)
                .help("Interfaces to add or remove"),
        )
        .disable_version_flag(true)
}

pub fn print_interfaces(interfaces: &[Interface]) {
    for iface in interfaces {
        let status = if iface.active { "active" } else { "inactive" };

        println!("{}: {}", iface.ifname, status);
    }
}
