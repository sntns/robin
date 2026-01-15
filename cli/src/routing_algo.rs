use clap::{Arg, Command};

/// Creates the CLI command for displaying or modifying the routing algorithm.
///
/// # Returns
/// - A `clap::Command` configured with:
///   - Name: `"routing_algo"`
///   - Alias: `"ra"`
///   - Short and long description: `"Display or modify the routing algorithm."`
///   - Usage override:
///       ```text
///       robctl [options] routing_algo|ra [options] [algo]
///       ```
///   - Single optional argument `"algo"`:
///       - The routing algorithm name (e.g., `"BATMAN_IV"`) to set
///       - Optional; if omitted, the current algorithm is displayed
///   - Version flag disabled
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
