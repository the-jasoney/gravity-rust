mod window;
mod events;
mod draw;
mod sim;

extern crate piston_window;
extern crate lazy_static;

use crate::window::create_window;
//use crate::events::handle_events;
use crate::sim::object::Object;
use sim::vec2::Vec2;
use sim::vec2;

use piston_window::ellipse::circle;
use piston_window::*;

use std::time::Instant;

const BALL_RADIUS: f64 = 10.0;

const WIDTH: u32 = 1200;
const HEIGHT: u32 = 500;

const FLOOR_Y: f64 = 490.0;

fn main() {
    let mut last_tick: Instant = Instant::now();

    let mut window: PistonWindow = create_window(WIDTH, HEIGHT);

    let mut gravity_object = Object {
        position: vec2!(0.0, HEIGHT/2),
        velocity: vec2!(500.0, -400),
        floor_y: FLOOR_Y,
        r: BALL_RADIUS,
        ..Object::default()
    };

    let ellipse_drawer = Ellipse::new([1.0; 4]);
    let line_drawer = Line::new([1.0; 4], 1.0);
    //let text_drawer = Text::new_color([1.0; 4], 15);

    while let Some(event) = window.next() {
        let dt: f64 = last_tick.elapsed().as_secs_f64();
        last_tick = Instant::now();

        gravity_object.update_position(dt);
        //println!("{}", gravity_object);

        window.draw_2d(&event, |context, graphics, _device| {
            clear([0.0; 4], graphics);

            ellipse_drawer.draw(
                circle(gravity_object.position.x, gravity_object.position.y - BALL_RADIUS, BALL_RADIUS),
                &DrawState {scissor: None, stencil: None, blend: None},
                context.transform,
                graphics
            );

            line_drawer.draw_arrow(
                [
                    gravity_object.position.x,
                    gravity_object.position.y,
                    gravity_object.position.x + gravity_object.velocity.x/3.0,
                    gravity_object.position.y + gravity_object.velocity.y/3.0 + gravity_object.velocity.y.signum()*10.0,
                ],
                10.0,
                &DrawState {scissor: None, stencil: None, blend: None},
                context.transform,
                graphics
            );

            line_drawer.draw(
                [
                    0.0,
                    FLOOR_Y,
                    WIDTH as f64,
                    FLOOR_Y
                ],
                &DrawState {scissor: None, stencil: None, blend: None},
                context.transform,
                graphics
            );
        });
    }
}
