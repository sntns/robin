use robin;

#[tokio::main]
async fn main() {
    let client = robin::RobinClient::new();
    let orig = client.originators().await.unwrap();
    let gateways = client.gateways().await.unwrap();

    for o in orig {
        println!("{:?}", o);
    }

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
}
