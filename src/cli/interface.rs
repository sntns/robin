use crate::Interface;

use clap::{Arg, Command};

/// Creates the CLI command for displaying or modifying batman-adv interfaces.
///
/// # Returns
/// - A `clap::Command` configured with:
///   - Name: `"interface"`
///   - Alias: `"if"`
///   - Short and long description: `"Display or modify the batman-adv interface settings."`
///   - Usage override:
///       ```text
///       robctl [options] interface|if [options] [add|del iface(s)]
///       robctl [options] interface|if [options] create [routing_algo|ra RA_NAME]
///       robctl [options] interface|if [options] destroy
///       ```
///   - Optional flags and arguments:
///       - `-M, --manual`: Disable automatic creation/destruction of batman-adv interface
///       - `action`: Command name, one of `add`, `a`, `del`, `d`, `create`, `c`, `destroy`, `D`
///       - `params`: Additional parameters, either interface names (for add/del) or routing algorithm name (for create)
///   - Version flag disabled
pub fn cmd_interfaces() -> Command {
    Command::new("interface")
        .alias("if")
        .about("Display or modify the batman-adv interface settings.")
        .long_about("Display or modify the batman-adv interface settings.")
        .override_usage(
            "\trobctl [options] interface|if [options] [add|del iface(s)]\n\
                    \trobctl [options] interface|if [options] create [routing_algo|ra RA_NAME]\n\
                    \trobctl [options] interface|if [options] destroy\n",
        )
        .arg(
            Arg::new("manual")
                .short('M')
                .help("Disable automatic creation/destruction of batman-adv interface")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("action")
                .index(1)
                .value_name("command")
                .value_parser(["add", "a", "del", "d", "create", "c", "destroy", "D"])
                .help("Command name:"),
        )
        .arg(
            Arg::new("params")
                .index(2)
                .value_name("parameters")
                .num_args(0..)
                .help("Interfaces (add/del) or routing algorithm (create)"),
        )
        .disable_version_flag(true)
}

/// Prints the list of batman-adv interfaces with their current status.
///
/// # Arguments
/// - `interfaces`: Slice of `Interface` structs, each containing:
///     - `ifname`: Name of the interface
///     - `active`: Boolean indicating whether the interface is active
///
/// # Behavior
/// - Prints each interface in the format: `"iface_name: active"` or `"iface_name: inactive"`.
pub fn print_interfaces(interfaces: &[Interface]) {
    for iface in interfaces {
        let status = if iface.active { "active" } else { "inactive" };

        println!("{}: {}", iface.ifname, status);
    }
}
