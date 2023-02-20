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
#[derive(Debug)]
pub struct Object {
    /// The position of the Object.
    pub position: Vec2,

    /// The velocity of the Object.
    pub velocity: Vec2,

    pub mass: f64
}

