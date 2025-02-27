use super::satellite::NeighboringSatelliteInformation;
use crate::simulation::{satellite::Satellite, tracking::create_satellites_map};
use rand::Rng;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct Contact {
    pub destination: u32,
    pub start_time: f64,
    pub end_time: f64,
    pub latency: f64,
}

pub struct SatelliteNetwork {
    satellites: HashMap<u32, Satellite>,
    connections_graph: HashMap<u32, Vec<Contact>>,
}

impl SatelliteNetwork {
    pub fn new() -> Self {
        Self {
            satellites: HashMap::new(),
            connections_graph: HashMap::new(),
        }
    }

    pub fn generate_satellite_network(&mut self, num_satellites: usize) -> () {
        // Retrieve the lazily-initialized thread-local random number generator.
        let mut rng = rand::thread_rng();
        let mut satellites = Vec::new();

        for num in 0..num_satellites {
            let id = num; // for now using num, no need for uuid
            let latitude = rng.gen_range(-90.0..90.0);
            let longitude = rng.gen_range(-180.0..180.0);
            let altitude = rng.gen_range(400.0..2000.0); // LEO range in km
            let velocity = 7.8; // Constant LEO orbital velocity
            satellites.push(Satellite::new(
                id as u32,
                (latitude, longitude),
                altitude,
                velocity,
            ));
        }
        self.add_satellites(&satellites);
        // satellites
    }

    /**
     * Recomputes all neighbors from scratch based on the latest state.
     */
    pub fn update_satellite_graph(&mut self) {
        println!("🔄 Updating satellite communication graph...");
        let updated_graph: HashMap<u32, Vec<Contact>> = create_satellites_map(&self.satellites);

        // Make ASYNC
        for (sat_id, new_contacts) in updated_graph {
            // own the values, no need to borrow for now
            if let Some(previous_contacts) = self.connections_graph.get_mut(&sat_id) {
                let old_destinations: HashSet<u32> =
                    previous_contacts.iter().map(|c| c.destination).collect();
                let new_destinations: HashSet<u32> =
                    new_contacts.iter().map(|c| c.destination).collect();

                let lost_connections: Vec<&u32> =
                    old_destinations.difference(&new_destinations).collect();
                let new_connections: Vec<&u32> =
                    new_destinations.difference(&old_destinations).collect();

                if !lost_connections.is_empty() {
                    println!(
                        "❌ Satellite {} lost connections with {:?}",
                        sat_id, lost_connections
                    );
                }
                if !new_connections.is_empty() {
                    println!(
                        "✅ Satellite {} established new connections with {:?}",
                        sat_id, new_connections
                    );
                }
                *previous_contacts = new_contacts; // update changed neighbors
            } else {
                println!(
                    "✅ New Satellite {} established in the network! With neighbors: {:?}",
                    sat_id,
                    new_contacts
                        .iter()
                        .map(|c| c.destination)
                        .collect::<Vec<_>>()
                );
                self.connections_graph.insert(sat_id, new_contacts); // add new sat as neighbors
            }
        }
    }

    pub fn get_satellites(&self) -> Vec<Satellite> {
        self.satellites.values().map(|sat| sat.clone()).collect()
    }

    fn add_satellites(&mut self, satellites: &Vec<Satellite>) {
        satellites.into_iter().for_each(|sat| {
            self.satellites.insert(sat.id, sat.clone());
        });
        println!("{:?}", satellites.len());
    }

    fn add_satellite(&mut self, sat: &Satellite) {
        self.satellites.insert(sat.id, sat.clone());
    }

    fn update_sat_positions(&mut self, time_step: f64) {
        self.satellites.values_mut().for_each(|sat| {
            sat.update_satellite_position(time_step);
        });
    }
}
