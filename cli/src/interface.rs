use robin::Interface;

use clap::{Arg, ArgMatches, Command};

pub fn cmd_interfaces() -> Command {
    Command::new("interface")
        .alias("if")
        .about("Display or modify the batman-adv interface settings")
        .long_about(
            "Display or modify the batman-adv interface settings.\n\
             \n\
             Usage:\n\
             \trobctl if                      # list interfaces\n\
             \trobctl if add iface1 ...       # add interfaces to mesh\n\
             \trobctl if del iface1 ...       # remove interfaces from mesh\n\
             \trobctl if create [routing_algo|ra ALGO]     # create batman-adv interface\n\
             \trobctl if destroy              # destroy batman-adv interface\n\
             \n\
             Options:\n\
             \t-M  Disable automatic creation/destruction of batman-adv interface",
        )
        .arg(
            Arg::new("manual")
                .short('M')
                .help("Disable automatic creation/destruction of batman-adv interface")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("action")
                .index(1)
                .value_parser(["add", "a", "del", "d", "create", "c", "destroy", "D"])
                .help("Action to perform"),
        )
        .arg(
            Arg::new("params")
                .index(2)
                .num_args(0..)
                .help("Interfaces (add/del) or create parameters (create)"),
        )
        .disable_version_flag(true)
}

pub fn print_interfaces(interfaces: &[Interface]) {
    for iface in interfaces {
        let status = if iface.active { "active" } else { "inactive" };

        println!("{}: {}", iface.ifname, status);
    }
}
