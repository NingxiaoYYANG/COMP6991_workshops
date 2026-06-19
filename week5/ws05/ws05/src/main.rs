use simulator_lib::directions::{coordinate::Coordinate, direction::Direction};
use simulator_lib::{start_server, Asteroid, Planet, PulsingGravitySource, DistanceBasedAsteroid};
fn main() {
    let planets = vec![
        Planet {
            coordinate: Coordinate::new(500, 500),
            weight: 50,
        },
    ];
    
    let pulsing_sources = vec![
        PulsingGravitySource::new(
            Coordinate::new(300, 300),
            30,  // base weight
            20,  // pulse amplitude
        ),
    ];
    
    let asteroids = vec![
        Asteroid {
            coordinate: Coordinate::new(250, 250),
            velocity: Direction { x: 30, y: -40 },
        },
        Asteroid {
            coordinate: Coordinate::new(750, 750),
            velocity: Direction { x: -30, y: 40 },
        },
    ];

    let distance_asteroids = vec![
        DistanceBasedAsteroid::new(
            Coordinate::new(100, 100),
            Direction { x: 20, y: 20 },
            100, // minimum distance before gravity affects it
        ),
    ];

    println!("Starting server. Open phys_simulation.html to see the simulation.");
    start_server("localhost:16991", planets, pulsing_sources, asteroids, distance_asteroids, 70);
}
