extern crate piston_window;

use piston_window::*;

pub fn create_window(w: u32, h: u32) -> piston_window::PistonWindow {
        WindowSettings::new("gravitati", [w, h])
            .exit_on_esc(true)
            .build()
            .unwrap()
}
