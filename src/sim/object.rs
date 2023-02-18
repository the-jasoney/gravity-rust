#![allow(dead_code)]
#![allow(unused_variables)]

use super::vec2::Vec2;
use crate::vec2;

/// Gravity constant.
/// Measured in pixels per second squared
const GRAVITY: Vec2 = vec2!(0, 800); // in pixels per second squared

/// Bounce Constant (aka deflective efficacy).
///
/// Represents how much energy is deflected off the floor back into the ball.
///
/// The following equation is always satisfied (in real life terms):
/// ```
/// 0.0 < BOUNCE_CONSTANT < 1.0
/// ```
const BOUNCE_CONSTANT: f64 = 0.60;

/// Friction constant.
///
/// Represents how much friction the ball experiences when touching the ground.
const FRICTION: f64 = 0.01;


/// Gravity-affected object
///
/// An object with a position and velocity.
/// Uses semi-implicit Euler's method to compute next postition and velocity from old position, velocity, and
/// acceleration.
pub struct Object {
    /// The position of the Object.
    pub position: Vec2,

    /// The velocity of the Object.
    pub velocity: Vec2,

    /// The Y-value of the floor.
    pub bound_left: f64,

    pub bound_right: f64,

    pub bound_top: f64,

    pub bound_bottom: f64
}

impl Object {
    /**
        Updates the position based on semi-implicit Euler's method
     */
    pub fn update_position(&mut self, dt: f64, object_locations: Vec<Vec2>) {
        self.detect_collision_with_wall();
        //self.detect_collision_with_object(object_locations);

        // next_velocity = current_velocity + dt*current_acceleration
        // acceleration in this case === GRAVITY
        self.velocity = self.velocity + GRAVITY*dt;

        // next_position = current_position + dt*next_velocity
        self.position = self.position + self.velocity*dt;
    }

    /// Detects collision with the walls and deflects the ball
    pub fn detect_collision_with_wall(&mut self) {
        if self.position.y > self.bound_bottom {
            self.velocity.y = -(self.velocity.y.abs() * BOUNCE_CONSTANT);
            self.position.y = self.bound_bottom - 0.05;

            self.velocity.x = self.velocity.x * (1.0 - FRICTION);
        } else if self.position.y < self.bound_top {
            self.velocity.y = self.velocity.y.abs() * BOUNCE_CONSTANT;
            self.position.y = self.bound_top + 0.05;

            self.velocity.x = self.velocity.x * (1.0 - FRICTION);
        }

        if self.position.x < self.bound_left {
            self.velocity.x = self.velocity.x.abs() * BOUNCE_CONSTANT;
            self.position.x = self.bound_left + 0.05;

            self.velocity.y = self.velocity.y * (1.0 - FRICTION);
        } else if self.position.x > self.bound_right {
            self.velocity.x = -(self.velocity.x.abs() * BOUNCE_CONSTANT);
            self.position.x = self.bound_right - 0.05;

            self.velocity.y = self.velocity.y * (1.0 - FRICTION);
        }
    }

    pub fn detect_collision_with_object(&mut self, objects: Vec<Vec2>) {
        for i in objects {
            if Vec2::dist_scalar(self.position, i) < 20.0 && i != self.position {
                self.velocity = -i * BOUNCE_CONSTANT;
                println!("collision!");
                break;
            }
        }
    }
}
