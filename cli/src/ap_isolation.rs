use clap::{Arg, Command};

pub fn cmd_ap_isolation() -> Command {
    Command::new("ap_isolation")
        .alias("ap")
        .about("Display or modify ap_isolation setting.")
        .long_about("Display or modify ap_isolation setting.")
        .override_usage("\trobctl [options] ap_isolation|ap [options] [0|1]\n")
        .arg(
            Arg::new("value")
                .value_name("0|1")
                .required(false)
                .value_parser(clap::value_parser!(u8).range(0..=1))
                .help("0 = disable ap_isolation, 1 = enable ap_isolation"),
        )
        .disable_version_flag(true)
}
