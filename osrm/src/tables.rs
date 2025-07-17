use derive_builder::Builder;
use serde::Deserialize;
pub(crate) use crate::point::Point;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct TableResponse {
    pub code: String,
    pub destinations: Vec<TableLocationEntry>,
    pub durations: Vec<Vec<Option<f64>>>,
    sources: Vec<TableLocationEntry>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct TableLocationEntry {
    hint: String,
    location: [f64; 2],
    name: String,
    distance: f64,
}

#[derive(Debug, Builder)]
pub struct TableRequest{
    pub sources: Vec<Point>,
    pub destinations: Vec<Point>
}