use std::fmt::{self, Display, Formatter, Result};

fn main() {
    let mut light = TrafficLight::new();
    println!("{:?}", light);
    light.turn_green();
    println!("{:?}", light);
}
// struct TrafficLight
#[derive(Debug)]
struct TrafficLight {
    color: TrafficLightColor,
}

impl TrafficLight {
    pub fn new() -> Self {
        Self {
            color: TrafficLightColor::Red,
        }
    }

    pub fn get_state(&self) -> &TrafficLightColor {
        &self.color
    }

    pub fn turn_green(&mut self) {
        self.color = TrafficLightColor::Green
    }
}

impl Display for TrafficLight {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut Formatter) -> Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "Traffic light is {}", self.color)
    }
}

// enum TrafficLightColor
#[derive(Debug)]
enum TrafficLightColor {
    Red,
    Yellow,
    Green,
}

impl Display for TrafficLightColor {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", &self)
    }
}

// struct HouseLight
#[derive(Debug)]
struct HouseLight {
    on: bool,
}

impl HouseLight {
    pub fn new() -> Self {
        Self { on: false }
    }
    pub fn get_state(&self) -> bool {
        self.on
    }
}

impl Display for HouseLight {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Houselight is {}", if self.on { "on" } else { "off" })
    }
}

// Function that applies to both TrafficLight and HouseLight
// For this we will use rust Trait
// Trait = Collection of function signatures
trait Light {
    // Light trait, has following function signatures
    // 1: get_name: takes &self and returns &str
    fn get_name(&self) -> &str;
}
