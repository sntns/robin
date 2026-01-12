use clap::{Arg, Command};

pub fn cmd_bridge_loop_avoidance() -> Command {
    Command::new("bridge_loop_avoidance")
        .alias("bl")
        .about("Display or modify bridge_loop_avoidance setting.")
        .long_about("Display or modify bridge_loop_avoidance setting.")
        .override_usage("\trobctl [options] bridge_loop_avoidance|bl [options] [0|1]\n")
        .arg(
            Arg::new("value")
                .value_name("0|1")
                .required(false)
                .value_parser(clap::value_parser!(u8).range(0..=1))
                .help("0 = disable bridge_loop_avoidance, 1 = enable bridge_loop_avoidance"),
        )
        .disable_version_flag(true)
}
