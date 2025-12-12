use robin;

#[tokio::main]
async fn main() {
    let client = robin::RobinClient::new();
    let orig = client.originators().await.unwrap();
    let gateways = client.gateways().await.unwrap();
    let gateway_mode = client.get_gw_mode().await.unwrap();

    println!("   Originator        last-seen (#/255) Nexthop           [outgoingIF]");
    for o in orig {
        println!(
            "{} {:} {:>7.3}s   ({:>3}) {:} [{}]",
            if o.is_best { "*" } else { " " },
            o.originator.to_string(),
            o.last_seen_ms as f64 / 1000.0,
            o.tq.unwrap_or(0),
            o.next_hop.to_string(),
            o.outgoing_if
        );
    }

    println!("\n\n");

    for g in gateways {
        println!(
            "{} {:02x?} via {:02x?} on {} down={:?} up={:?} throughput={:?} tq={:?}",
            if g.is_best { "*" } else { " " },
            g.mac_addr,
            g.router,
            g.outgoing_if,
            g.bandwidth_down,
            g.bandwidth_up,
            g.throughput,
            g.tq
        );
    }

    println!("\n\n");

    match gateway_mode.mode {
        robin::GwMode::Off => {
            println!("off");
        }

        robin::GwMode::Client => {
            println!(
                "client (selection class: {}.{} MBit)",
                gateway_mode.sel_class / 10,
                gateway_mode.sel_class % 10
            );
        }

        robin::GwMode::Server => {
            let d_major = gateway_mode.bandwidth_down / 10;
            let d_minor = gateway_mode.bandwidth_down % 10;
            let u_major = gateway_mode.bandwidth_up / 10;
            let u_minor = gateway_mode.bandwidth_up % 10;

            println!(
                "server (announced bw: {}.{}/{}.{} MBit)",
                d_major, d_minor, u_major, u_minor
            );
        }
        _ => {}
    }
}
