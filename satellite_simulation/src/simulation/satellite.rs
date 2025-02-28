use rand::Rng;

use crate::common::{calculate_angular_velocity, calculate_euclid_distance};

#[allow(warnings)]
#[derive(Debug, Clone)]
pub struct Satellite {
    pub id: u32,
    pub position: (f64, f64), // latitude and longitude
    pub altitude: f64,        // in km
    pub velocity: f64,
    pub storage_on_board: f64,
    // for now we are assuming there is only one point in ground aka one point for downlink
    pub distance_to_ground: Option<f64>,
    pub energy_efficiency: f64,
    pub time_to_downlink: f64,
    pub communication_window: f64,
    pub orbital_radius: f64,
    pub past_positions: Vec<(f64, f64)>, // used for storage of history
}

const G: f64 = 6.674e-11; // G Constant
const EARTH_MASS: f64 = 5.972e24; // Mass of our Earth in KG
const EARTH_RADIUS: f64 = 6_371_000.0; // Earth radius in meters
const MAX_ONBOARD_STORAGE: f64 = 10000.0; // will adjust on chosen storage config
const MAX_ENERGY_CAPACITY: f64 = 100.0; // will adjust on sensor params
const COMMUNICATION_RANGE: f64 = 1000.0; // in km

impl Satellite {
    pub fn new(id: u32, position: (f64, f64), altitude: f64, velocity: f64) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            id,
            position,
            altitude,
            velocity,
            storage_on_board: rng.gen_range(500.0..MAX_ONBOARD_STORAGE),
            distance_to_ground: None,
            energy_efficiency: rng.gen_range(50.0..MAX_ENERGY_CAPACITY),
            time_to_downlink: 0.0,
            communication_window: 0.0,
            orbital_radius: EARTH_RADIUS + (altitude * 1000.0),
            past_positions: Vec::<(f64, f64)>::new(),
        }
    }

    /**
    *  Larger time_step = bigger movement per update, meaning we "fast forward" the simulation.
       Smaller time_step = finer-grained movement, giving a smoother simulation.
       Example: time_step = 10.0 (meaning we simulate 10 seconds of movement in one update)
    */
    pub fn update_satellite_position(&mut self, time_step: f64) {
        // Keep track of 1000 past sat positions for prediction-based heuristic
        self.past_positions.push(self.position);
        if self.past_positions.len() > 1000 {
            self.past_positions.remove(0);
        }

        // Change SAT's position using angular velocity (change in radians) and time_step
        let angular_velocity = self.get_current_speed() / self.orbital_radius;
        let theta_change = angular_velocity * time_step; // Change in angle over time step

        // Update position using simple orbital model
        self.position.0 = self.orbital_radius * (self.position.0.to_radians() + theta_change).cos();
        self.position.1 = self.orbital_radius * (self.position.1.to_radians() + theta_change).sin();
    }

    pub fn update_satellite_altitude(&mut self, altitude_diff: f64) {
        self.altitude += altitude_diff;
        self.orbital_radius = EARTH_RADIUS + (self.altitude * 1000.0);
    }

    pub(crate) fn calculate_relay_score(&self, ground_position: (f64, f64),) -> f64 {
        let distance_to_ground = self.get_distance_from_ground();
        let storage_score = 1.0/(self.storage_on_board + 1.0); // avoids division by zero
        let energy_avail_score = 1.0/(self.energy_efficiency + 1.0);
        let time_to_downlink_score = self.time_to_downlink;
        let communication_window_score = 1.0/(self.communication_window + 1.0);

        (distance_to_ground * 1.5) + (storage_score * 0.5) + (energy_avail_score * 0.3) +
        (time_to_downlink_score * 1.0) + (communication_window_score * 0.2)
    }

    // This represents the height above Earth's surface in meters.
    pub(crate) fn get_distance_from_ground(&self) -> f64 {
        self.altitude * 1000.0 // km to meters
    }

    // Update when using docker for storage availability per satellite
    // Aka we are gonna have alocated space of 10MB per satellite and
    // exchange information when the Ground Control asks for downlink.
    // Generation of data will be simulated for now. Later step would be
    // To use one satellite as the collection satellite and have it be
    // hooked to reading information from the different sensors of a
    // STM32F3/L4 board and collect that info every 30 secs.
    pub(crate) fn get_satellite_storage(&self) -> f64 {
        let mut rng = rand::thread_rng();
        rng.gen_range(500.0..MAX_ONBOARD_STORAGE)
    }

    pub(crate) fn get_current_speed(&self) -> f64 {
        (G * EARTH_MASS / self.orbital_radius).sqrt()
    }

    /**
     * Estimates how much time the satellite has until it reaches the closest point to
     * the ground station in its circular orbit.
     */
    pub(crate) fn update_time_to_downlink(&mut self, ground_position: (f64, f64)) {
        let angular_velocity =
            calculate_angular_velocity(self.get_current_speed(), self.orbital_radius); // w = v / r

        // Satellite latitude -> radians
        let theta_sat_lat = self.position.0.to_radians();
        let theta_sat_lon = self.position.1.to_radians();
        // Ground station latitude -> radians
        let theta_ground_lat = ground_position.0.to_radians();
        let theta_ground_lon = ground_position.1.to_radians();

        // Angular distance between satellite and ground using spherical law of cosines
        let delta_theta = ((theta_sat_lat.sin() * theta_ground_lat.sin())
            + (theta_sat_lat.cos()
                * theta_ground_lat.cos()
                * (theta_sat_lon - theta_ground_lon).cos()))
        .acos();
        let delta_theta_future = ((theta_sat_lat.sin() * theta_ground_lat.sin())
            + ((theta_sat_lat.cos() * theta_ground_lat.cos()
                + (theta_sat_lon + angular_velocity * 10.0)
                - theta_ground_lon)
                .cos()))
        .acos();
        let is_approaching = delta_theta_future < delta_theta;
        if is_approaching {
            // time until closest approach: t = theta / w
            self.time_to_downlink = delta_theta / angular_velocity;
        } else {
            let full_orbit_time = 2.0 * std::f64::consts::PI / angular_velocity;
            let time_until_next_phase = full_orbit_time - (delta_theta / angular_velocity);
            self.time_to_downlink = time_until_next_phase;
        }
    }

    /**
    *  Computes the total duration the satellite spends inside the communication range.
       Uses the subtended angle of the communication arc and orbital angular velocity
       to determine time in range.
    */
    pub(crate) fn update_communication_window(&mut self) {
        let subtended_angle = COMMUNICATION_RANGE * 1000.0 / self.orbital_radius;
        let total_angle = 2.0 * subtended_angle;

        // w = v / r
        let angular_velocity =
            calculate_angular_velocity(self.get_current_speed(), self.orbital_radius);
        self.communication_window = total_angle / angular_velocity; // Time in seconds
    }
}

pub struct NeighboringSatelliteInformation {
    pub id: u32,                           // id of the neighboring satellite
    pub distance_from_source: Option<f64>, // distance from the source satellite
    pub speed: Option<f64>,                // speed of the neighboring satellite
    pub distance_from_ground: Option<f64>, // distance of the current satellite from the ground receiver
    pub available_storage: Option<f64>,    // how much memory storage is available on board
}

impl NeighboringSatelliteInformation {
    pub fn new(id: &u32) -> Self {
        NeighboringSatelliteInformation {
            id: id.clone(),
            distance_from_source: None,
            speed: None,
            distance_from_ground: None,
            available_storage: None,
        }
    }

    pub fn get_distance_from_ground(&mut self, distance: f64) {
        self.distance_from_ground = Some(distance);
    }

    pub fn get_distance_from_source(&mut self, distance: f64) {
        self.distance_from_source = Some(distance);
    }

    pub fn get_available_storage(&mut self, available_storage: f64) {
        self.available_storage = Some(available_storage);
    }

    pub fn get_speed(&mut self, speed: f64) {
        self.speed = Some(speed);
    }
}
