use serde::Deserialize;
use crate::point::Point;

pub struct TripRequest {
    pub points : Vec<Point>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct TripResponse {

}