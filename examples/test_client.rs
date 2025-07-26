use std::time::Instant;
use osrm_binding::algorithm::Algorithm;
use osrm_binding::osrm_engine::OsrmEngine;
use osrm_binding::point::Point;
use osrm_binding::tables::TableRequest;

fn main() {
    dotenvy::dotenv().expect(".env file could not be read");
    let path = std::env::var("OSRM_TEST_DATA_PATH_MLD")
        .expect("Environment variable OSRM_TEST_DATA_PATH_MLD must be defined with a french map");
    let engine = OsrmEngine::new(&*path, Algorithm::MLD).expect("Failed to initialize OSRM engine");

    let start = Instant::now();  // Capture start time
    (0..100).for_each(|_| {
        let request = TableRequest {
            sources: vec![
                Point { longitude: 2.3522, latitude: 48.8566 } // Paris
            ],
            destinations: vec![
                Point { longitude: 5.3698, latitude: 43.2965 }, // Marseille
                Point { longitude: 4.8357, latitude: 45.7640 }  // Lyon
            ]
        };
        let _ = engine.table(request).expect("Table request failed");
    });
    let duration = start.elapsed();  // Calculate the elapsed time
    println!("Time taken for 100 tables: {:?}", duration);

    let start = Instant::now();  // Capture start time
    (0..100).for_each(|_| {
        let _ = engine.simple_route(Point { longitude: 2.3522, latitude: 48.8566 }, Point {  longitude: 5.3698, latitude: 43.2965 }).expect("route request failed");
    });
    let duration = start.elapsed();  // Calculate the elapsed time
    println!("Time taken for 100 simple route: {:?}", duration);

}