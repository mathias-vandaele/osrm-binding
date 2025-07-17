// osrm/src/lib.rs

mod algorithm;
mod errors;
mod tables;
mod trip;
mod point;

use crate::errors::OsrmError;
use crate::tables::{TableRequest, TableResponse};
use crate::trip::{TripRequest, TripResponse};

pub struct OsrmEngine {
    instance: osrm_sys::Osrm,
}

impl OsrmEngine {

    pub fn new(base_path: &str, algorithm : algorithm::Algorithm) -> Result<Self, OsrmError> {
        let osrm = osrm_sys::Osrm::new(base_path, algorithm.as_str()).map_err( |_|  OsrmError::Initialization )?;
        Ok(OsrmEngine {
            instance: osrm,
        })
    }

    pub fn table(&self, table_request: TableRequest) -> Result<TableResponse, OsrmError> {
        let len_sources = table_request.sources.len();
        let len_destinations = table_request.destinations.len();
        if len_sources == 0 || len_destinations == 0 {
            return Err(OsrmError::InvalidTableArgument);
        }
        let sources_index: &[usize]  = &(0..(len_sources)).collect::<Vec<usize>>()[..];
        let destination_index: &[usize]  = &(len_sources..(len_sources+len_destinations)).collect::<Vec<usize>>()[..];
        let coordinates: &[(f64, f64)] =  &[table_request.sources, table_request.destinations].concat().iter().map( |s| (s.longitude, s.latitude) ).collect::<Vec<(f64, f64)>>()[..];
        let result = self.instance.table(coordinates, Some(sources_index), Some(destination_index)).map_err( |e| OsrmError::FfiError(e))?;
        serde_json::from_str::<TableResponse>(&result).map_err(|e| OsrmError::JsonParse(e))
    }

    pub fn trip(&self, trip_request: TripRequest) -> Result<TripResponse, OsrmError> {
        let len = trip_request.points.len();
        if len == 0 {
            return Err(OsrmError::InvalidTableArgument);
        }
        let coordinates: &[(f64, f64)] =  &trip_request.points.iter().map( |p|  (p.longitude, p.latitude) ).collect::<Vec<(f64, f64)>>()[..];
        let result = self.instance.trip(coordinates).map_err( |e| OsrmError::FfiError(e))?;
        serde_json::from_str::<TripResponse>(&result).map_err(|e| OsrmError::JsonParse(e))
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Import OsrmEngine, TableRequest, etc.
    use crate::algorithm::Algorithm;
    use crate::tables::Point;
    #[test]
    fn it_calculates_a_table_successfully() {
        dotenvy::dotenv().expect(".env file could not be read");
        let path = std::env::var("OSRM_TEST_DATA_PATH")
            .expect("Environment variable OSRM_TEST_DATA_PATH must be defined with a french map");
        let engine = OsrmEngine::new(&*path, Algorithm::MLD).expect("Failed to initialize OSRM engine");

        let request = TableRequest {
            sources: vec![
                Point { longitude: 2.3522, latitude: 48.8566 } // Paris
            ],
            destinations: vec![
                Point { longitude: 5.3698, latitude: 43.2965 }, // Marseille
                Point { longitude: 4.8357, latitude: 45.7640 }  // Lyon
            ]
        };

        let response = engine.table(request).expect("Table request failed");

        println!("{:?}", response.durations);

        assert_eq!(response.code, "Ok");
        assert_eq!(response.durations.len(), 1, "Should have 1 row for 1 source");
        assert_eq!(response.durations[0].len(), 2, "Should have 2 columns for 2 destinations");
        assert!(response.durations[0][0].is_some(), "Paris-Marseille duration should exist");
        assert!(response.durations[0][1].is_some(), "Paris-Lyon duration should exist");

        println!("Durations from Paris to [Marseille, Lyon]: {:?}", response.durations[0]);
    }
}