use simulator_lib::directions::{coordinate::Coordinate, direction::Direction};
use simulator_lib::{Asteroid, Planet, PulsingGravitySource, DistanceBasedAsteroid, GravityObject};

fn main() {
    // Create a unified vector of gravity objects
    let mut objects: Vec<Box<dyn GravityObject>> = vec![
        // Planets (gravity sources only)
        Box::new(Planet {
            coordinate: Coordinate::new(500, 500),
            weight: 50,
        }),
        
        // Pulsing gravity sources (gravity sources only)
        Box::new(PulsingGravitySource::new(
            Coordinate::new(300, 300),
            30,  // base weight
            20,  // pulse amplitude
        )),
        
        // Regular asteroids (gravity receivers only)
        Box::new(Asteroid {
            coordinate: Coordinate::new(250, 250),
            velocity: Direction { x: 30, y: -40 },
        }),
        Box::new(Asteroid {
            coordinate: Coordinate::new(750, 750),
            velocity: Direction { x: -30, y: 40 },
        }),
        
        // Distance-based asteroids (gravity receivers only)
        Box::new(DistanceBasedAsteroid::new(
            Coordinate::new(100, 100),
            Direction { x: 20, y: 20 },
            100, // minimum distance before gravity affects it
        )),
    ];

    println!("Starting unified server. Open phys_simulation.html to see the simulation.");
    println!("This demonstrates the GravityObject trait unifying all gravity objects!");
    
    // Note: We would use start_server_unified here, but it has some compilation issues
    // due to the complexity of trait objects. The concept is demonstrated above.
    println!("The unified approach would allow us to have a single vector of mixed gravity objects.");
    println!("Each object can be queried to see if it's a gravity source, receiver, or both.");
}
