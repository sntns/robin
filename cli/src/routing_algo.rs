use clap::{Arg, Command};

pub fn cmd_routing_algo() -> Command {
    Command::new("routing_algo")
        .alias("ra")
        .about("Display or modify the routing algorithm.")
        .long_about("Display or modify the routing algorithm.")
        .override_usage("\trobctl [options] routing_algo|ra [options] [algo]\n")
        .arg(
            Arg::new("algo")
                .required(false)
                .help("Routing algorithm name (e.g. BATMAN_IV)"),
        )
        .disable_version_flag(true)
}
