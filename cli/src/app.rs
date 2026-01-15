use crate::aggregation::cmd_aggregation;
use crate::ap_isolation::cmd_ap_isolation;
use crate::bridge_loop_avoidance::cmd_bridge_loop_avoidance;
use crate::gateways::cmd_gateways;
use crate::gw_mode::cmd_gw_mode;
use crate::interface::cmd_interfaces;
use crate::neighbors::cmd_neighbors;
use crate::originators::cmd_originators;
use crate::routing_algo::cmd_routing_algo;
use crate::transglobal::cmd_transglobal;
use crate::translocal::cmd_translocal;
use clap::{Arg, Command};

/// Builds the command-line interface (CLI) for `robctl`.
///
/// This function sets up the main CLI structure using `clap`, including global options
/// and all subcommands.
///
/// # Global Options
/// - `--meshif`, `-m` : Specify the batman-adv mesh interface to operate on (default: `bat0`).
/// - `--version`, `-v` : Print the `robctl` version and the batman-adv kernel module version (if loaded).
///
/// # Subcommands
/// - `neighbors` (`n`) : Display the neighbor table.
/// - `gateways` (`gwl`) : Display the list of gateways.
/// - `gw_mode` (`gw`) : Display or modify the gateway mode.
/// - `originators` (`o`) : Display the originator table.
/// - `translocal` (`tl`) : Display local translation table.
/// - `transglobal` (`tg`) : Display global translation table.
/// - `interface` (`if`) : Display or modify batman-adv interface settings.
/// - `ap_isolation` (`ap`) : Display or modify AP isolation setting.
/// - `aggregation` (`ag`) : Display or modify aggregation setting.
/// - `bridge_loop_avoidance` (`bl`) : Display or modify bridge loop avoidance setting.
/// - `routing_algo` (`ra`) : Display or modify the routing algorithm.
///
/// # Returns
/// A `clap::Command` ready to parse command-line arguments.
///
/// # Example
/// ```no_run
/// let cli = build_cli();
/// let matches = cli.get_matches();
/// ```
pub fn build_cli() -> Command {
    Command::new("robctl")
        //.subcommand_required(true)
        .arg_required_else_help(true)
        .about("Rust implementation of batctl, B.A.T.M.A.N. advanced control tool")
        .arg(
            Arg::new("meshif")
                .long("meshif")
                .short('m')
                .value_name("IFACE")
                .help("Batman-adv mesh interface to operate on (default: bat0)"),
        )
        .arg(
            Arg::new("version")
                .short('v')
                .long("version")
                .help("Print robctl version and batman-adv module version (if loaded)")
                .action(clap::ArgAction::SetTrue),
        )
        .subcommand(cmd_neighbors())
        .subcommand(cmd_gateways())
        .subcommand(cmd_gw_mode())
        .subcommand(cmd_originators())
        .subcommand(cmd_translocal())
        .subcommand(cmd_transglobal())
        .subcommand(cmd_interfaces())
        .subcommand(cmd_ap_isolation())
        .subcommand(cmd_aggregation())
        .subcommand(cmd_bridge_loop_avoidance())
        .subcommand(cmd_routing_algo())
}
