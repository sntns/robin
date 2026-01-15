use clap::{Arg, Command};

/// Creates the CLI command for querying or modifying the aggregation setting.
///
/// # Returns
/// - A `clap::Command` configured with:
///   - Name: `"aggregation"`
///   - Alias: `"ag"`
///   - Short and long description: `"Display or modify aggregation setting."`
///   - Usage override: `robctl [options] aggregation|ag [options] [0|1]`
///   - Optional argument `value`:
///     - Type: `u8`
///     - Allowed values: `0` (disable) or `1` (enable)
///     - Help: `"0 = disable aggregation, 1 = enable aggregation"`
///
/// # Notes
/// - If no `value` is provided, the command can be used to display the current aggregation state.
/// - Version flag is disabled for this command.
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
