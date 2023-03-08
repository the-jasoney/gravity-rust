use crate::Object;
use crate::{vec2, Vec2};

use super::segment::Segment;
use super::util::reduce2d;

/// Gravity constant.
/// Measured in pixels per second squared
const GRAVITY: Vec2 = vec2!(0, 1000); // in pixels per second squared

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

pub struct Solver {
    pub objects: Vec<Object>,
    pub bound_left: f64,
    pub bound_right: f64,
    pub bound_top: f64,
    pub bound_bottom: f64,
}

impl Solver {
    pub fn new(bound_left: f64, bound_right: f64, bound_top: f64, bound_bottom: f64) -> Solver {
        Solver {
            objects: vec![],
            bound_left,
            bound_right,
            bound_top,
            bound_bottom,
        }
    }

    pub fn add_object(&mut self, position: Vec2, velocity: Vec2, mass: f64) {
        self.objects.push(Object {
            position,
            velocity,
            next_position: position,
            next_velocity: velocity,
            mass,
        });
    }

    pub fn solve_collision_walls(&mut self) {
        for i in &mut self.objects {
            // check for collision with wall
            if i.position.y > self.bound_bottom {
                i.next_velocity.y = -(i.velocity.y.abs() * BOUNCE_CONSTANT);
                i.next_position.y = self.bound_bottom - 0.05;

                i.next_velocity.x = i.velocity.x * (1.0 - FRICTION);
            } else if i.position.y < self.bound_top {
                i.next_velocity.y = i.velocity.y.abs() * BOUNCE_CONSTANT;
                i.next_position.y = self.bound_top + 0.05;

                i.next_velocity.x = i.velocity.x * (1.0 - FRICTION);
            }

            if i.position.x < self.bound_left {
                i.next_velocity.x = i.velocity.x.abs() * BOUNCE_CONSTANT;
                i.next_position.x = self.bound_left + 0.05;

                i.next_velocity.y = i.velocity.y * (1.0 - FRICTION);
            } else if i.position.x > self.bound_right {
                i.next_velocity.x = -(i.velocity.x.abs() * BOUNCE_CONSTANT);
                i.next_position.x = self.bound_right - 0.05;

                i.next_velocity.y = i.velocity.y * (1.0 - FRICTION);
            }
        }
    }

    pub fn solve_euler(&mut self, dt: f64) {
        for i in &mut self.objects {
            // next_velocity = current_velocity + dt*current_acceleration
            // acceleration in this case === GRAVITY
            i.next_velocity = i.next_velocity + GRAVITY * dt;

            // next_position = current_position + dt*next_velocity
            i.next_position = i.next_position + i.next_velocity * dt;
        }
    }

    pub fn get_collisions(&self) -> Vec<usize> {
        let mut collisions: Vec<Vec<usize>> = vec![];
        for (idx, i) in self.objects.iter().enumerate() {
            let mut collision_vec: Vec<usize> = vec![idx];
            let i_segment = Segment::from((i.position, i.next_position)).extend(i.mass);

            for (jdx, j) in self.objects.iter().enumerate() {
                let j_segment = Segment::from((j.position, j.next_position));

                if let Some(_) = Segment::intersection(&i_segment, &j_segment) {
                    // this function only checks for
                    // collisions, and we can throw
                    // away the value
                    if !collision_vec.contains(&jdx) {
                        collision_vec.push(jdx);
                    }
                }
            }

            if collision_vec.len() > 1 {
                collisions.push(collision_vec);
            }
        }

        reduce2d(collisions)
    }

    pub fn solve_collision_objects(&mut self) {
        /*
            solve collisions by going over each object and checking if any other object will collide with it within the next
            frame. if it is found to be colliding, then its index will be added to an array. this array, after finishing
            the loop for each object, will be added to an array

        */
        let mut collisions: Vec<Vec<usize>> = vec![];
    }

    #[allow(non_snake_case)]
    pub fn collide(&mut self, mut idx: usize, mut jdx: usize) {
        if idx > jdx {
            [idx, jdx] = [jdx, idx];
        }
        let (head, tail) = self.objects.split_at_mut(idx + 1);

        let object1 = &mut head[idx];
        let object2 = &mut tail[jdx - idx - 1];

        let (m1, m2) = (object1.mass, object2.mass);
        let M = m1 + m2;
        let (x1, x2) = (object1.position, object2.position);

        //object1.velocity = object1.velocity - ((2.0*m2)/M) *

        println!("collision between {:?} and {:?}", object1, object2);
    }

    pub fn resolve(&mut self) {
        for i in &mut self.objects {
            i.position = i.next_position;
            i.velocity = i.next_velocity;
        }
    }

    pub fn solve_all(&mut self, dt: f64) {
        //self.solve_collision_objects();
        self.solve_collision_walls();
        self.solve_euler(dt);
        let collisions = self.get_collisions();

        if collisions.len() > 0 {
            println!("{:?}", self.get_collisions());
        }
        self.resolve()
    }

    pub fn solve_for_x_seconds(
        &self,
        position: Vec2,
        velocity: Vec2,
        mass: f64,
        t: i64,
    ) -> Vec<Vec2> {
        let tx5 = t * 10;

        let mut result: Vec<Vec2> = vec![];

        let mut i = Object {
            position,
            velocity,
            next_position: vec2!(),
            next_velocity: vec2!(),
            mass,
        };

        for _ in 0..tx5 {
            if i.position.y > self.bound_bottom {
                i.velocity.y = -(i.velocity.y.abs() * BOUNCE_CONSTANT);
                i.position.y = self.bound_bottom - 0.05;

                i.velocity.x = i.velocity.x * (1.0 - FRICTION);
            } else if i.position.y < self.bound_top {
                i.velocity.y = i.velocity.y.abs() * BOUNCE_CONSTANT;
                i.position.y = self.bound_top + 0.05;

                i.velocity.x = i.velocity.x * (1.0 - FRICTION);
            }

            if i.position.x < self.bound_left {
                i.velocity.x = i.velocity.x.abs() * BOUNCE_CONSTANT;
                i.position.x = self.bound_left + 0.05;

                i.velocity.y = i.velocity.y * (1.0 - FRICTION);
            } else if i.position.x > self.bound_right {
                i.velocity.x = -(i.velocity.x.abs() * BOUNCE_CONSTANT);
                i.position.x = self.bound_right - 0.05;

                i.velocity.y = i.velocity.y * (1.0 - FRICTION);
            }

            // next_velocity = current_velocity + dt*current_acceleration
            // acceleration in this case === GRAVITY
            i.velocity += GRAVITY * (t as f64) * 0.01;

            // next_position = current_position + dt*next_velocity
            i.position += i.velocity * (t as f64) * 0.01;

            result.push(i.position);
        }
        result
    }
}
