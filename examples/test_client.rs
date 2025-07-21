use osrm_binding::algorithm::Algorithm;
use osrm_binding::osrm_engine::OsrmEngine;
use osrm_binding::point::Point;
use osrm_binding::tables::TableRequest;

fn main() {
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
}