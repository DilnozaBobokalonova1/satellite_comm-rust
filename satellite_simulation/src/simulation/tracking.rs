use crate::{
    common::{
        calculate_euclid_distance, calculate_future_satellite_position, SPEED_OF_LIGHT,
        TIME_LOOKAHEAD_SECS,
    },
    simulation::satellite::Satellite,
};
use std::{cmp::min, collections::HashMap};

use super::{graph::Contact, satellite::NeighboringSatelliteInformation};

const COMMUNICATION_RANGE: f64 = 1000.0;

/**
 * Tracking module is meant to calculate a map of the current satellites position in space.
 * It is also meant to update the map every X seconds. Lets say 3 seconds for now.
 * And it is also the one to give to the satellite its current list of neighbors.
 * But it is not the one that tells the satellite which neighbor is best to pick for relay.
 * That would be the pathfinding module's job that uses a certain set of heuristics aka routing!
 */

/**
 * Now lets assument the tracking information gets updated every 3 seconds. We have to call an update on the existing graph
 * but also keep track of who is in the network vss who is out of the network that was initially in it.
 * Is that efficient? Can a certain module just be the one tracking that information in the background?
 *
 * So then, should tracking become a struct or what's the way to just have it be a module?
 * Because where do we save the neighboring information?
 */

/*
    Create a map of nearby satellites for each satellite in the Hashmap.
    What we mean by graph is that we have each satellite id containing information
    of all the satellites within its network in the radius of COMMUNICATION_RANGE.
    * Information meaning each Neighbor's ID and a distance to that neighbor.
    * The improvement to this would be to also have orbital information and the information
    * of where each of those neighbors is heading as to make a decision of where we'd
    * like to send the information for ground downlink or receive information to act
    * as the sender or the receiver.

    Hence, HashMap<u32, Vec<NeighboringSatelliteInformation>> is the return information on satellites_map creation

*/

/**
 * Computes a dynamic map of contacts between satellites. Each satellite
 * has a list of Contact objects representing future communication windows.
 */
pub fn create_satellites_map(satellites: &HashMap<u32, Satellite>) -> HashMap<u32, Vec<Contact>> {
    let mut connections = HashMap::new();

    for (id1, sat1) in satellites.iter() {
        let mut contact_list: Vec<Contact> = Vec::new();

        for (id2, sat2) in satellites.iter() {
            if id1 == id2 {
                continue;
            }
            // Calculate the current Euclidean distance btw sat1 & sat2
            let distance_btw_sats = calculate_euclid_distance(&sat1.position, &sat2.position);
            let predicted_distance = predict_future_distance(sat1, sat2, TIME_LOOKAHEAD_SECS);

            // If within communication range (now or in 10 seconds), create a contact
            if distance_btw_sats <= COMMUNICATION_RANGE || predicted_distance <= COMMUNICATION_RANGE
            {
                let start_time = sat1.time_to_downlink;
                let end_time = start_time + sat1.communication_window;
                let latency =
                    (distance_btw_sats / SPEED_OF_LIGHT).min(predicted_distance / SPEED_OF_LIGHT); // Speed of light delay in ms

                contact_list.push(Contact {
                    destination: *id2,
                    start_time,
                    end_time,
                    latency,
                });
            }
        }
        connections.insert(*id1, contact_list);
    }

    connections
}

fn predict_future_distance(sat1: &Satellite, sat2: &Satellite, time_step: f64) -> f64 {
    calculate_euclid_distance(
        &(&calculate_future_satellite_position(sat1, time_step)),
        &(calculate_future_satellite_position(sat2, time_step)),
    )
}
