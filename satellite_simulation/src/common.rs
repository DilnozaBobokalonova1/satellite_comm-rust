use crate::simulation::satellite::Satellite;

pub fn calculate_euclid_distance(pos1: &(f64, f64), pos2: &(f64, f64)) -> f64 {
    let dx = pos1.0 - pos2.0;
    let dy = pos1.1 - pos2.1;

    (dx.powi(2) + dy.powi(2)).sqrt()
}

// Since, v = w * r => w = v / r
pub fn calculate_angular_velocity(velocity: f64, radius: f64) -> f64 {
    return velocity / radius;
}

pub fn calculate_future_satellite_position(satellite: &Satellite, time_step: f64) -> (f64, f64) {
    let angular_velocity = satellite.get_current_speed() / satellite.orbital_radius;
    let theta_change = angular_velocity * time_step;
    let predicted_x =
        satellite.orbital_radius * (satellite.position.0.to_radians() + theta_change).cos();
    let predicted_y =
        satellite.orbital_radius * (satellite.position.1.to_radians() + theta_change).sin();

    (predicted_x, predicted_y)
}

pub const TIME_LOOKAHEAD_SECS: f64 = 10.0;
pub const SPEED_OF_LIGHT: f64 = 299_792.458;
