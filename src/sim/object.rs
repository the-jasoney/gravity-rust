use std::fmt;
use derivative::Derivative;

use super::vec2::Vec2;
use crate::vec2;

const GRAVITY: Vec2 = vec2!(0, 250);
const BOUNCE_CONSTANT: f64 = 0.75;
const FRICTION: f64 = 0.40; // smaller value = more friction

#[derive(Derivative)]
#[derivative(Default)]
pub struct Object {
    #[derivative(Default(value = "vec2!(0, 0)"))]
    pub position: Vec2,

    #[derivative(Default(value = "vec2!(0, 0)"))]
    pub velocity: Vec2,

    #[derivative(Default(value = "0.0"))]
    pub floor_y: f64,

    #[derivative(Default(value = "0.0"))]
    pub r: f64,

    /*
    #[derivative(Default(value = "false"))]
    pub hasCollided: bool
    */
}

impl Object {
    /**
        Updates the position based on semi-implicit Euler's method
     */
    pub fn update_position(&mut self, dt: f64) {
        self.detect_collision();

        // next_velocity = current_velocity + dt*current_acceleration
        // acceleration in this case === GRAVITY
        self.velocity = self.velocity + GRAVITY*dt;

        // next_position = current_position + dt*next_velocity
        self.position = self.position + self.velocity*dt;
    }

    pub fn detect_collision(&mut self) {
        if self.position.y > self.floor_y {

            self.velocity.y = -(self.velocity.y.abs() * BOUNCE_CONSTANT);
            self.position.y = self.floor_y - 0.05;

            self.velocity.x = self.velocity.x * FRICTION
        }
    }
}



impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Object; Location = {}, Velocity = {}. Y-value of floor: {}", self.position, self.velocity, self.floor_y)
    }
}
