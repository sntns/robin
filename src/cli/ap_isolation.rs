use clap::{Arg, Command};

/// Creates the CLI command for querying or modifying the AP isolation setting.
///
/// # Returns
/// - A `clap::Command` configured with:
///   - Name: `"ap_isolation"`
///   - Alias: `"ap"`
///   - Short and long description: `"Display or modify ap_isolation setting."`
///   - Usage override: `robctl [options] ap_isolation|ap [options] [0|1]`
///   - Optional argument `value`:
///     - Type: `u8`
///     - Allowed values: `0` (disable) or `1` (enable)
///     - Help: `"0 = disable ap_isolation, 1 = enable ap_isolation"`
///
/// # Notes
/// - If no `value` is provided, the command can be used to display the current AP isolation state.
/// - Version flag is disabled for this command.
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
