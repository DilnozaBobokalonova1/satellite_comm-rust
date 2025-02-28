use super::tracking::Contact;
use crate::simulation::{satellite::Satellite, tracking::create_satellites_map};
use rand::Rng;
use core::f64;
use std::collections::{HashMap, HashSet};

pub struct SatelliteNetwork {
    satellites_dict: HashMap<u32, Satellite>,
    satellites_network: HashMap<u32, Vec<Contact>>,
}

impl SatelliteNetwork {
    pub fn new() -> Self {
        Self {
            satellites_dict: HashMap::new(),
            satellites_network: HashMap::new(),
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
    pub fn update_satellite_network(&mut self) {
        println!("🔄 Updating satellite communication graph...");
        let updated_graph: HashMap<u32, Vec<Contact>> = create_satellites_map(&self.satellites_dict);

        // Make ASYNC
        for (sat_id, new_contacts) in updated_graph {
            // own the values, no need to borrow for now
            if let Some(previous_contacts) = self.satellites_network.get_mut(&sat_id) {
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
                self.satellites_network.insert(sat_id, new_contacts); // add new sat as neighbors
            }
        }
    }

    pub fn find_best_relay(&self, source_satellite_id: u32, ground_position: (f64, f64)) -> Option<u32> {
        let source = self.satellites_dict.get(&source_satellite_id)?;
        let mut best_relay = None;
        let mut best_score = f64::MAX;

        for (_, sat) in &self.satellites_dict {
            if sat.id == source_satellite_id {
                continue;
            }
            let score = sat.calculate_relay_score(ground_position);
            if score < best_score {
                best_score = score;
                best_relay = Some(sat.id);
            }
        }
        best_relay
    }

    fn add_satellites(&mut self, satellites: &Vec<Satellite>) {
        satellites.into_iter().for_each(|sat| {
            self.satellites_dict.insert(sat.id, sat.clone());
        });
        println!("{:?}", satellites.len());
    }

    fn add_satellite(&mut self, sat: &Satellite) {
        self.satellites_dict.insert(sat.id, sat.clone());
    }

    fn update_sat_positions(&mut self, time_step: f64) {
        self.satellites_dict.values_mut().for_each(|sat| {
            sat.update_satellite_position(time_step);
        });
    }
}
