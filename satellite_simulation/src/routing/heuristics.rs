use core::f64;

use crate::{common::calculate_euclid_distance, simulation::satellite::Satellite};

/**
 * The thing with finding the next best satellite to communicate my information to the ground is based on multiple factors:
 *      1. The satellite's direction, speed, altitude to ground.
 *      2. The satellite's available storage on-board.
 *      3. The satellite's actual availability to do the relay.
 *      4. What else?
 */

/**
 * Returns an optional id of the best relay satellite for downlink. If no such relay available, the
 * calling function's satellite would be its own relay. (Define what that would mean in this case.)
 */
pub fn find_best_relay(
    source: &Satellite,
    satellites: &Vec<Satellite>,
    ground_position: (f64, f64),
) -> Option<u32> {
    let mut best_relay_satellite_id = None;
    let mut best_distance = f64::MAX;

    for sat in satellites {
        if sat.id == source.id {
            continue;
        }
        let dist_to_ground = calculate_euclid_distance(&sat.position, &ground_position);
        let source_to_ground = calculate_euclid_distance(&source.position, &ground_position);

        // The best relay is the satellite that is closer to the ground station than the source
        if dist_to_ground < source_to_ground && dist_to_ground < best_distance {
            best_distance = dist_to_ground;
            best_relay_satellite_id = Some(sat.id)
        }
    }

    best_relay_satellite_id
}
