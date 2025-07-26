use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Waypoint {
    pub hint: String,
    pub location: [f64; 2],
    pub name: String,
    pub distance: f64,
}