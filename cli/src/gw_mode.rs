use crate::robin::{GatewayInfo, GwMode, RobinError};

use clap::{Arg, Command};

pub fn cmd_gw_mode() -> Command {
    Command::new("gw_mode")
        .alias("gw")
        .about("Display or modify the gateway mode.")
        .long_about("Display or modify the gateway mode.")
        .override_usage("robctl gw_mode|gw [off|client|server] [sel_class|bandwidth]\n")
        .arg(
            Arg::new("mode")
                .value_name("mode")
                .required(false)
                .help("Gateway mode (off|client|server)"),
        )
        .arg(
            Arg::new("param")
                .value_name("sel_class|bandwidth")
                .required(false)
                .help("Gateway parameter (selection class or bandwidth)"),
        )
        //.disable_help_flag(true)
        .disable_version_flag(true)
}

pub fn print_gw(info: &GatewayInfo) {
    match info.mode {
        GwMode::Off => {
            println!("off");
        }
        GwMode::Client => {
            if info.algo == "BATMAN_V" {
                println!(
                    "client (selection class: {}.{} MBit)",
                    info.sel_class / 10,
                    info.sel_class % 10
                );
            } else {
                println!("client (selection class: {} MBit)", info.sel_class,);
            }
        }
        GwMode::Server => {
            let down = info.bandwidth_down;
            let up = info.bandwidth_up;

            println!(
                "server (announced bw: {}.{}/{}.{} MBit)",
                down / 10,
                down % 10,
                up / 10,
                up % 10
            );
        }
        GwMode::Unknown => {
            println!("unknown");
        }
    }
}

pub fn parse_gw_param(
    mode: GwMode,
    param: &str,
) -> Result<(Option<u32>, Option<u32>, Option<u32>), RobinError> {
    match mode {
        GwMode::Off => Ok((None, None, None)),
        GwMode::Client => {
            let sel_class = param.parse::<u32>().map_err(|e| {
                RobinError::Parse(format!("Invalid sel_class '{}': {:?}", param, e))
            })?;
            Ok((None, None, Some(sel_class)))
        }
        GwMode::Server => {
            let parts: Vec<&str> = param.split('/').collect();
            let parse_value = |s: &str| -> Result<u32, RobinError> {
                let s = s.trim().to_lowercase();
                if s.ends_with("kbit") {
                    Ok(s.trim_end_matches("kbit").parse::<u32>().map_err(|e| {
                        RobinError::Parse(format!("Invalid bandwidth '{}': {:?}", s, e))
                    })?)
                } else if s.ends_with("mbit") {
                    let val = s.trim_end_matches("mbit").parse::<u32>().map_err(|e| {
                        RobinError::Parse(format!("Invalid bandwidth '{}': {:?}", s, e))
                    })?;
                    Ok(val * 1000)
                } else {
                    Ok(s.parse::<u32>().map_err(|e| {
                        RobinError::Parse(format!("Invalid bandwidth '{}': {:?}", s, e))
                    })?)
                }
            };

            let down = parse_value(parts[0])?;
            let up = if let Some(u) = parts.get(1) {
                Some(parse_value(u)?)
            } else {
                Some(down / 5)
            };

            Ok((Some(down), up, None))
        }
        GwMode::Unknown => Err(RobinError::NotFound("Unknown mode".to_string())),
    }
}
