# osrm-rs

Rust bindings for [OSRM (Open Source Routing Machine)](http://project-osrm.org/), providing an idiomatic and type-safe interface to access OSRM core functionalities (`route`, `table`, `trip`) from Rust.

## 🚀 Features

- 🗺️ Calculate routes, trips, and distance/duration tables using OSRM
- 🦀 Fully written in Rust
- 💡 Simple and safe API for high-performance routing
- 🧪 Includes integration tests for route and table APIs

## 📦 Installation

## 🛠️ Usage

### Initialization

```rust
let engine = OsrmEngine::new("/path/to/france-latest.osrm", Algorithm::MLD)
.expect("Failed to initialize OSRM engine");
```

### Route Calculation

```rust
let request = RouteRequestBuilder::default()
.points(vec![
    Point { longitude: 2.3522, latitude: 48.8566 }, // Paris
    Point { longitude: 5.3698, latitude: 43.2965 }, // Marseille
    ])
.build()
.unwrap();

let result = engine.route(request).unwrap();
println!("{:?}", result.routes.first().unwrap());
```


### Table (Distance/Duration Matrix)

```rust
let request = TableRequest {
    sources: vec![Point { longitude: 2.3522, latitude: 48.8566 }],
    destinations: vec![
        Point { longitude: 5.3698, latitude: 43.2965 },
        Point { longitude: 4.8357, latitude: 45.7640 },
    ],
};

let response = engine.table(request).unwrap();
println!("{:?}", response.durations);
```



### Simple Route

```rust
    let result = engine.simple_route(
        Point { longitude: 2.3522, latitude: 48.8566 },
        Point { longitude: 5.3698, latitude: 43.2965 },
    ).unwrap();
    
    println!("Duration: {}s, Distance: {}m", result.durations, result.distance);
```


### Trip API

```rust
    let request = TripRequest {
        points: vec![
        Point { longitude: 2.3522, latitude: 48.8566 },
        Point { longitude: 4.8357, latitude: 45.7640 },
        Point { longitude: 5.3698, latitude: 43.2965 },
        ],
    };
    
    let trip = engine.trip(request).unwrap();
    println!("{:?}", trip);
```


## 🔬 Tests

To run the tests:

export OSRM_TEST_DATA_PATH="/absolute/path/to/france-latest.osrm"  
cargo test

Make sure your `.osrm` file is correctly prepared using `osrm-extract` and `osrm-contract`.

## 📖 License

MIT

> **Note**: This library assumes the user has a working `osrm-c-api` compiled and accessible. You must generate `.osrm` files yourself using `osrm-extract` and `osrm-contract`.

## ✨ Contribution

Feel free to open issues or PRs if you want to improve performance, add more OSRM API bindings, or enhance usability.

---

Made with ❤️ in Rust.
