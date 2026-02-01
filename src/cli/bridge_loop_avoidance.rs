use clap::{Arg, Command};

/// Creates the CLI command for querying or modifying the bridge loop avoidance setting.
///
/// # Returns
/// - A `clap::Command` configured with:
///   - Name: `"bridge_loop_avoidance"`
///   - Alias: `"bl"`
///   - Short and long description: `"Display or modify bridge_loop_avoidance setting."`
///   - Usage override: `robctl [options] bridge_loop_avoidance|bl [options] [0|1]`
///   - Optional argument `value`:
///     - Type: `u8`
///     - Allowed values: `0` (disable) or `1` (enable)
///     - Help: `"0 = disable bridge_loop_avoidance, 1 = enable bridge_loop_avoidance"`
///
/// # Notes
/// - If no `value` is provided, the command can be used to display the current bridge loop avoidance state.
/// - Version flag is disabled for this command.
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
