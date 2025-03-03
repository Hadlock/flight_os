struct Plane {
    speed: f64,
    heading: f64,
    altitude: f64,
}

const BLACK: u8 = 0;

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

    fn handle_key_press(&mut self) {
        // Stub function for handling key presses
        /*
        if is_key_down(KeyCode::Up) {
            self.altitude += 10.0;
        }
        if is_key_down(KeyCode::Down) {
            self.altitude -= 10.0;
        }
        if is_key_down(KeyCode::Left) {
            self.heading -= 5.0;
        }
        if is_key_down(KeyCode::Right) {
            self.heading += 5.0;
        }
        if is_key_down(KeyCode::PageUp) {
            self.speed += 1.0;
        }
        if is_key_down(KeyCode::PageDown) {
            self.speed -= 1.0;
        }
        */
    }

    fn display_hud(&self) {
        /* stub function for displaying HUD 
        let hud_text = format!(
            "Speed: {:.2} knots | Heading: {:.2} degrees | Altitude: {:.2} feet",
            self.speed, self.heading, self.altitude
        );
        draw_text(&hud_text, 10.0, 390.0, 20.0, WHITE);
        */
    }

}


fn setup_display() {
    // Stub function for setting up X Windows
}

fn load_map_files() {
    // Stub function for loading map files from stdin
}

fn sleep_for_interval() {
    // stub function for sleeping for a short interval
    //sleep(Duration::from_secs_f64(0.02));
}

fn update_display() {
    // Stub function for updating display
}

fn clear_background(_color: u8) {
    // Stub: do nothing
}

fn handle_key_press() {
    // Stub: do nothing
} 
async fn next_frame() {
    // Stub: do nothing
}

async fn simloop(plane: &mut Plane) {
    loop {
        clear_background(BLACK);
        sleep_for_interval();
        plane.calculate_angles();
        update_display();
        handle_key_press();        
        plane.handle_key_press();
        plane.update_physics();
        plane.display_hud();

        next_frame().await;
    }
}

pub fn main() {
    setup_display();
    load_map_files();

    let mut plane = Plane::new();
    
    flight_os::graphics::draw_line((100, 150), (220, 150));
    flight_os::graphics::draw_line((110, 160), (210, 160));
    
    // Run async simulation loop using the executor.
    flight_os::executor::block_on(simloop(&mut plane));
}
