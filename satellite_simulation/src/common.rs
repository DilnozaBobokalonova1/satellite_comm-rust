pub fn calculate_euclid_distance(pos1: &(f64, f64), pos2: &(f64, f64)) -> f64 {
    let dx = pos1.0 - pos2.0;
    let dy = pos1.1 - pos2.1;

    (dx.powi(2) + dy.powi(2)).sqrt()
}

// Since, v = w * r => w = v / r
pub fn calculate_angular_velocity(velocity: f64, radius: f64) -> f64 {
    return velocity / radius;
}
