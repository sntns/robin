use clap::{Arg, Command};

pub fn cmd_aggregation() -> Command {
    Command::new("aggregation")
        .alias("ag")
        .about("Display or modify aggregation setting.")
        .long_about("Display or modify aggregation setting.")
        .override_usage("\trobctl [options] aggregation|ag [options] [0|1]\n")
        .arg(
            Arg::new("value")
                .value_name("0|1")
                .required(false)
                .value_parser(clap::value_parser!(u8).range(0..=1))
                .help("0 = disable aggregation, 1 = enable aggregation"),
        )
        .disable_version_flag(true)
}
