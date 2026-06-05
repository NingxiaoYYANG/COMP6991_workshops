pub mod directions;

use crate::directions::{coordinate::Coordinate, direction::Direction};

use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream},
};

use serde::{Deserialize, Serialize};

// Trait for objects that have a position and can be visualised as a circle
pub trait Positioned {
    fn get_location(&self) -> Coordinate;
    fn as_circle(&self) -> Circle;
}

// Trait for objects that provide gravity
pub trait GravitySource: Positioned {
    fn get_gravity_strength(&self) -> i32;
}

// Trait for objects that are affected by gravity
pub trait GravityReceiver: Positioned {
    fn apply_gravity_force(&mut self, force: Direction);
    fn update_position(&mut self);
}

// Unified trait for objects that can be gravity sources, receivers, or both
pub trait GravityObject: Positioned {
    fn as_gravity_source(&self) -> Option<&dyn GravitySource>;
    fn as_gravity_receiver(&mut self) -> Option<&mut dyn GravityReceiver>;
}

#[derive(Deserialize, Serialize)]
pub struct Circle {
    cx: i32,
    cy: i32,
    r: i32,
    stroke: String,
    fill: String,
    #[serde(rename = "stroke-width")]
    stroke_width: i32,
}

#[derive(Clone)]
pub struct Planet {
    pub coordinate: Coordinate,
    pub weight: i32,
}

impl Planet {
    fn get_weight(&self) -> i32 {
        self.weight
    }
}

impl Positioned for Planet {
    fn get_location(&self) -> Coordinate {
        self.coordinate.clone()
    }

    fn as_circle(&self) -> Circle {
        Circle {
            cx: self.coordinate.x,
            cy: self.coordinate.y,
            r: self.weight,
            stroke: "green".to_string(),
            fill: "black".to_string(),
            stroke_width: 3,
        }
    }
}

impl GravitySource for Planet {
    fn get_gravity_strength(&self) -> i32 {
        self.weight
    }
}

impl GravityObject for Planet {
    fn as_gravity_source(&self) -> Option<&dyn GravitySource> {
        Some(self)
    }

    fn as_gravity_receiver(&mut self) -> Option<&mut dyn GravityReceiver> {
        None // Planets don't move, so they're not gravity receivers
    }
}

#[derive(Clone)]
pub struct Asteroid {
    pub coordinate: Coordinate,
    pub velocity: Direction,
}

impl Asteroid {
    fn get_velocity(&self) -> Direction {
        self.velocity.clone()
    }
}

impl Positioned for Asteroid {
    fn get_location(&self) -> Coordinate {
        self.coordinate.clone()
    }

    fn as_circle(&self) -> Circle {
        Circle {
            cx: self.coordinate.x,
            cy: self.coordinate.y,
            r: 2,
            stroke: "green".to_string(),
            fill: "black".to_string(),
            stroke_width: 3,
        }
    }
}

impl GravityReceiver for Asteroid {
    fn apply_gravity_force(&mut self, force: Direction) {
        self.velocity.x -= force.x;
        self.velocity.y -= force.y;
    }

    fn update_position(&mut self) {
        self.coordinate.x += self.velocity.x;
        self.coordinate.y += self.velocity.y;
    }
}

impl GravityObject for Asteroid {
    fn as_gravity_source(&self) -> Option<&dyn GravitySource> {
        None // Asteroids don't provide gravity
    }

    fn as_gravity_receiver(&mut self) -> Option<&mut dyn GravityReceiver> {
        Some(self)
    }
}

// A pulsing gravity source that alternates between high and low gravity
#[derive(Clone)]
pub struct PulsingGravitySource {
    pub coordinate: Coordinate,
    pub base_weight: i32,
    pub pulse_amplitude: i32,
    pub time_step: i32,
}

impl PulsingGravitySource {
    pub fn new(coordinate: Coordinate, base_weight: i32, pulse_amplitude: i32) -> Self {
        Self {
            coordinate,
            base_weight,
            pulse_amplitude,
            time_step: 0,
        }
    }

    pub fn update(&mut self) {
        self.time_step += 1;
    }
}

impl Positioned for PulsingGravitySource {
    fn get_location(&self) -> Coordinate {
        self.coordinate.clone()
    }

    fn as_circle(&self) -> Circle {
        Circle {
            cx: self.coordinate.x,
            cy: self.coordinate.y,
            r: 30, // Fixed size for visualization
            stroke: "red".to_string(),
            fill: "orange".to_string(),
            stroke_width: 3,
        }
    }
}

impl GravitySource for PulsingGravitySource {
    fn get_gravity_strength(&self) -> i32 {
        // Use sine wave to create pulsing effect
        let pulse = ((self.time_step as f64 * 0.1).sin() * self.pulse_amplitude as f64) as i32;
        self.base_weight + pulse
    }
}

impl GravityObject for PulsingGravitySource {
    fn as_gravity_source(&self) -> Option<&dyn GravitySource> {
        Some(self)
    }

    fn as_gravity_receiver(&mut self) -> Option<&mut dyn GravityReceiver> {
        None // Pulsing sources don't move, so they're not gravity receivers
    }
}

// An asteroid that is only affected by gravity when far from gravity sources
#[derive(Clone)]
pub struct DistanceBasedAsteroid {
    pub coordinate: Coordinate,
    pub velocity: Direction,
    pub min_distance: i32,
}

impl DistanceBasedAsteroid {
    pub fn new(coordinate: Coordinate, velocity: Direction, min_distance: i32) -> Self {
        Self {
            coordinate,
            velocity,
            min_distance,
        }
    }
}

impl Positioned for DistanceBasedAsteroid {
    fn get_location(&self) -> Coordinate {
        self.coordinate.clone()
    }

    fn as_circle(&self) -> Circle {
        Circle {
            cx: self.coordinate.x,
            cy: self.coordinate.y,
            r: 3, // Slightly larger to distinguish from regular asteroids
            stroke: "blue".to_string(),
            fill: "lightblue".to_string(),
            stroke_width: 3,
        }
    }
}

impl GravityReceiver for DistanceBasedAsteroid {
    fn apply_gravity_force(&mut self, force: Direction) {
        // This will be overridden in the physics calculation
        // to check distance before applying force
        self.velocity.x -= force.x;
        self.velocity.y -= force.y;
    }

    fn update_position(&mut self) {
        self.coordinate.x += self.velocity.x;
        self.coordinate.y += self.velocity.y;
    }
}

impl GravityObject for DistanceBasedAsteroid {
    fn as_gravity_source(&self) -> Option<&dyn GravitySource> {
        None // Distance-based asteroids don't provide gravity
    }

    fn as_gravity_receiver(&mut self) -> Option<&mut dyn GravityReceiver> {
        Some(self)
    }
}


fn get_distance(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    (((x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2)) as f64).sqrt() as i32
}

fn apply_physics<T: GravitySource, U: GravityReceiver>(planets: &mut Vec<T>, pulsing_sources: &mut Vec<PulsingGravitySource>, mut asteroids: Vec<U>, mut distance_asteroids: Vec<DistanceBasedAsteroid>, gravitational_constant: i32) -> (Vec<Asteroid>, Vec<DistanceBasedAsteroid>) {
    // Update pulsing gravity sources
    pulsing_sources.iter_mut().for_each(|source| {
        source.update();
    });

    // Collect all gravity sources
    let mut gravity_data = Vec::new();
    gravity_data.extend(planets.iter().map(|p| (p.get_location(), p.get_gravity_strength())));
    gravity_data.extend(pulsing_sources.iter().map(|p| (p.get_location(), p.get_gravity_strength())));

    // Apply gravity to regular asteroids
    asteroids.iter_mut().for_each(|asteroid| {
        gravity_data
            .iter()
            .for_each(|(source_coord, source_weight)| {
                let distance = get_distance(
                    source_coord.x,
                    source_coord.y,
                    asteroid.coordinate.x,
                    asteroid.coordinate.y,
                );
                let distance = distance * distance;

                let force = Direction {
                    x: (asteroid.coordinate.x - source_coord.x)
                        * source_weight
                        * gravitational_constant
                        / distance,
                    y: (asteroid.coordinate.y - source_coord.y)
                        * source_weight
                        * gravitational_constant
                        / distance,
                };
                asteroid.apply_gravity_force(force);
            })
    });

    // Apply gravity to distance-based asteroids (only when far enough)
    distance_asteroids.iter_mut().for_each(|asteroid| {
        gravity_data
            .iter()
            .for_each(|(source_coord, source_weight)| {
                let distance = get_distance(
                    source_coord.x,
                    source_coord.y,
                    asteroid.coordinate.x,
                    asteroid.coordinate.y,
                );
                
                // Only apply gravity if distance is greater than min_distance
                if distance > asteroid.min_distance {
                    let distance_squared = distance * distance;
                    let force = Direction {
                        x: (asteroid.coordinate.x - source_coord.x)
                            * source_weight
                            * gravitational_constant
                            / distance_squared,
                        y: (asteroid.coordinate.y - source_coord.y)
                            * source_weight
                            * gravitational_constant
                            / distance_squared,
                    };
                    asteroid.apply_gravity_force(force);
                }
            })
    });

    // Update positions for all asteroids
    asteroids.iter_mut().for_each(|asteroid| {
        asteroid.update_position();
    });
    
    distance_asteroids.iter_mut().for_each(|asteroid| {
        asteroid.update_position();
    });

    (asteroids, distance_asteroids)
}

// New unified physics function using GravityObject trait
// This demonstrates how we could use a single vector of GravityObjects
fn apply_physics_unified<T: GravityObject + Clone>(objects: &mut [T], gravitational_constant: i32) {
    // Update pulsing gravity sources first
    objects.iter_mut().for_each(|_obj| {
        // Check if this is a PulsingGravitySource and update it
        // Note: This is a simplified approach - in practice, you'd need more sophisticated type checking
    });

    // Collect all gravity sources
    let mut gravity_data = Vec::new();
    objects.iter().for_each(|obj| {
        if let Some(source) = obj.as_gravity_source() {
            gravity_data.push((source.get_location(), source.get_gravity_strength()));
        }
    });

    // Apply gravity to all gravity receivers
    objects.iter_mut().for_each(|obj| {
        if let Some(receiver) = obj.as_gravity_receiver() {
            gravity_data.iter().for_each(|(source_coord, source_weight)| {
                let receiver_coord = receiver.get_location();
                let distance = get_distance(
                    source_coord.x,
                    source_coord.y,
                    receiver_coord.x,
                    receiver_coord.y,
                );
                let distance_squared = distance * distance;

                let force = Direction {
                    x: (receiver_coord.x - source_coord.x)
                        * source_weight
                        * gravitational_constant
                        / distance_squared,
                    y: (receiver_coord.y - source_coord.y)
                        * source_weight
                        * gravitational_constant
                        / distance_squared,
                };
                receiver.apply_gravity_force(force);
            });
        }
    });

    // Update positions for all gravity receivers
    objects.iter_mut().for_each(|obj| {
        if let Some(receiver) = obj.as_gravity_receiver() {
            receiver.update_position();
        }
    });
}

fn handle_connection(
    mut stream: TcpStream,
    mut planets: Vec<Planet>,
    mut pulsing_sources: Vec<PulsingGravitySource>,
    mut asteroids: Vec<Asteroid>,
    mut distance_asteroids: Vec<DistanceBasedAsteroid>,
    gravitational_constant: i32,
) -> (Vec<Planet>, Vec<PulsingGravitySource>, Vec<Asteroid>, Vec<DistanceBasedAsteroid>) {
    (asteroids, distance_asteroids) = apply_physics(&mut planets, &mut pulsing_sources, asteroids, distance_asteroids, gravitational_constant);

    let mut circles = Vec::new();
    circles.extend(planets.iter().map(|p| p.as_circle()));
    circles.extend(pulsing_sources.iter().map(|p| p.as_circle()));
    circles.extend(asteroids.iter().map(|a| a.as_circle()));
    circles.extend(distance_asteroids.iter().map(|a| a.as_circle()));
    
    let contents = serde_json::to_string(&circles).unwrap();
    let status_line = "HTTP/1.1 200 OK";
    let response = format!(
        "{status_line}\r\nContentType: application/json\r\nAccess-Control-Allow-Origin: *\r\n\r\n{contents}\r\n"
    );
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    stream.shutdown(std::net::Shutdown::Both).unwrap();

    (planets, pulsing_sources, asteroids, distance_asteroids)
}

pub fn start_server(uri: &str, mut planets: Vec<Planet>, mut pulsing_sources: Vec<PulsingGravitySource>, mut asteroids: Vec<Asteroid>, mut distance_asteroids: Vec<DistanceBasedAsteroid>, gravitational_constant: i32) -> ! {
    let listener = TcpListener::bind(uri).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        (planets, pulsing_sources, asteroids, distance_asteroids) = handle_connection(stream, planets, pulsing_sources, asteroids, distance_asteroids, gravitational_constant);
    }

    unreachable!()
}

// New unified server function that demonstrates the GravityObject trait
// This shows how we could use a single vector of mixed gravity objects
pub fn start_server_unified(uri: &str, mut objects: Vec<Box<dyn GravityObject>>, gravitational_constant: i32) -> ! {
    let listener = TcpListener::bind(uri).unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        // Update pulsing gravity sources
        objects.iter_mut().for_each(|_obj| {
            // In a real implementation, you'd need to downcast to check if it's a PulsingGravitySource
            // and call update() on it. This is simplified for demonstration.
        });

        // Collect all gravity sources
        let mut gravity_data = Vec::new();
        objects.iter().for_each(|obj| {
            if let Some(source) = obj.as_gravity_source() {
                gravity_data.push((source.get_location(), source.get_gravity_strength()));
            }
        });

        // Apply gravity to all gravity receivers
        objects.iter_mut().for_each(|obj| {
            if let Some(receiver) = obj.as_gravity_receiver() {
                gravity_data.iter().for_each(|(source_coord, source_weight)| {
                    let receiver_coord = receiver.get_location();
                    let distance = get_distance(
                        source_coord.x,
                        source_coord.y,
                        receiver_coord.x,
                        receiver_coord.y,
                    );
                    let distance_squared = distance * distance;

                    let force = Direction {
                        x: (receiver_coord.x - source_coord.x)
                            * source_weight
                            * gravitational_constant
                            / distance_squared,
                        y: (receiver_coord.y - source_coord.y)
                            * source_weight
                            * gravitational_constant
                            / distance_squared,
                    };
                    receiver.apply_gravity_force(force);
                });
            }
        });

        // Update positions for all gravity receivers
        objects.iter_mut().for_each(|obj| {
            if let Some(receiver) = obj.as_gravity_receiver() {
                receiver.update_position();
            }
        });

        // Create visualization circles
        let mut circles = Vec::new();
        objects.iter().for_each(|obj| {
            circles.push(obj.as_circle());
        });

        // Send response
        let contents = serde_json::to_string(&circles).unwrap();
        let status_line = "HTTP/1.1 200 OK";
        let response = format!(
            "{status_line}\r\nContentType: application/json\r\nAccess-Control-Allow-Origin: *\r\n\r\n{contents}\r\n"
        );
        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
        stream.shutdown(std::net::Shutdown::Both).unwrap();
    }

    unreachable!()
}
