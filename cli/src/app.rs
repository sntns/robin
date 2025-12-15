use clap::Command;

use crate::gateways::cmd_gateways;
use crate::gw_mode::cmd_gw_mode;
use crate::neighbors::cmd_neighbors;
use crate::originators::cmd_originators;
use crate::transglobal::cmd_transglobal;
use crate::translocal::cmd_translocal;

pub fn build_cli() -> Command {
    Command::new("robctl")
        //.disable_help_flag(true)
        //.disable_version_flag(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .about("Rust implementation of batctl, B.A.T.M.A.N. advanced control tool")
        .subcommand(cmd_neighbors())
        .subcommand(cmd_gateways())
        .subcommand(cmd_gw_mode())
        .subcommand(cmd_originators())
        .subcommand(cmd_translocal())
        .subcommand(cmd_transglobal())
}
