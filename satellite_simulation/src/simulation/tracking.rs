use crate::{common::calculate_euclid_distance, simulation::satellite::Satellite};
use std::collections::HashMap;

use super::satellite::NeighboringSatelliteInformation;

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

pub fn create_satellites_map(
    satellites: &HashMap<u32, Satellite>,
) -> HashMap<u32, Vec<NeighboringSatelliteInformation>> {
    let mut connections = HashMap::new();

    for (id1, sat1) in satellites.iter() {
        let mut neighbors: Vec<NeighboringSatelliteInformation> = Vec::new();

        for (id2, sat2) in satellites.iter() {
            if id1 == id2 {
                continue;
            }
            // Current Euclidean distance
            let distance_btw_sats = calculate_euclid_distance(&sat1.position, &sat2.position);

            // Compute angular velocity for orbital motion
            let angular_velocity = sat2.get_current_speed() / sat2.orbital_radius; // w = v / r
            let theta_change = angular_velocity * 10.0; // Predicting 10 sec ahead

            // Predict future position using orbital movement
            let predicted_x =
                sat2.orbital_radius * (sat2.position.0.to_radians() + theta_change).cos();
            let predicted_y =
                sat2.orbital_radius * (sat2.position.1.to_radians() + theta_change).sin();
            let predicted_position = (predicted_x, predicted_y);

            // 10 sec lookahead to predict future position of sat2 based on velocity * time
            let predicted_distance = calculate_euclid_distance(&sat1.position, &predicted_position);

            if distance_btw_sats <= COMMUNICATION_RANGE || predicted_distance <= COMMUNICATION_RANGE
            {
                let mut neighbor_info = NeighboringSatelliteInformation::new(id2);

                // FIX THIS, EITHER UPDATE OR GET BASED ON WHERE TO STORE INFO
                neighbor_info.get_speed(sat2.get_current_speed());
                neighbor_info.get_available_storage(sat2.get_satellite_storage());
                neighbor_info.get_distance_from_source(distance_btw_sats);
                neighbor_info.get_distance_from_ground(sat2.get_distance_from_ground());
                neighbors.push(neighbor_info);
            }
        }
        connections.insert(*id1, neighbors);
    }

    connections
}
