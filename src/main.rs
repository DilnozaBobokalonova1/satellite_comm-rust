use std::collections::HashMap;

use clap::{Arg, ArgMatches, Command};
use rand::Rng;
use simulation::{graph::SatelliteNetwork, satellite::Satellite};

mod common;
mod communication;
mod routing;
mod security;
mod simulation;
mod storage;

#[tokio::main]
async fn main() {
    let matches = Command::new("Satellite Simulator")
        .version("1.0")
        .about("Simulates a satellite communication network!")
        .arg(
            Arg::new("num-satellites")
                .short('n')
                .long("num-satellites")
                .help("Number of satellites to simulate")
                .default_value("10")
                .value_parser(clap::value_parser!(usize)),
        )
        .get_matches();

    let num_satellites: usize = *matches.get_one::<usize>("num-satellites").unwrap_or(&5);

    // Create a network of satellites by first generating them then creating a graph and
    // updating their respective positions in the graph
    let mut network = SatelliteNetwork::new();
    network.generate_satellite_network(num_satellites);
    network.update_satellite_graph();

    // let connections: HashMap<u32, Vec<u32>> = find_nearby_satellites(&network.satellites);

    // for (sat_id, neighbors) in connections {
    //     println!("Satellite {} can communicate with {:?}", sat_id, neighbors);
    // }
}
