/**
 * config.rs
 *
 * Parses the configuration file for autotemp.
 */

struct FanState {
    last_temp: f32,
    last_speed: u8,
    last_change_at: Instant,
}

#[derive(Deserialize)]
pub struct Config {
    pub fan: FanConfig,
    pub curve: Vec<Point>,
}
