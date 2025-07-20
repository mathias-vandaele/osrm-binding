use derive_builder::Builder;
use serde::Deserialize;
use crate::point::Point;

#[derive(Debug, Builder)]
pub struct TripRequest {
    pub points : Vec<Point>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct TripResponse {

}