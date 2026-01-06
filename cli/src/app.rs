use clap::{Arg, Command};

use crate::gateways::cmd_gateways;
use crate::gw_mode::cmd_gw_mode;
use crate::interface::cmd_interfaces;
use crate::neighbors::cmd_neighbors;
use crate::originators::cmd_originators;
use crate::transglobal::cmd_transglobal;
use crate::translocal::cmd_translocal;

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
}
