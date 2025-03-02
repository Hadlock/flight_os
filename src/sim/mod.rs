struct Plane {
    speed: f64,
    heading: f64,
    altitude: f64,
}

impl Plane {
    fn new() -> Self {
        Plane {
            speed: 130.0,
            heading: 0.0,
            altitude: 1000.0,
        }
    }

    fn update_physics(&mut self) {
        // Stub function for updating physics
    }

    fn calculate_angles(&mut self) {
        // Stub function for calculating angles
    }

    }



pub fn main() {

    let mut plane = Plane::new();


    
}
