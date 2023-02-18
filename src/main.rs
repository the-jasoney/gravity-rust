mod window;
mod sim;

extern crate piston_window;
extern crate lazy_static;

use crate::window::create_window;
use crate::sim::object::Object;
use sim::vec2::Vec2;
use sim::vec2;

use piston_window::ellipse::circle;
use piston_window::*;
use piston_window::Input::{Move, Button};
use piston_window::Motion::MouseCursor;
use piston_window::Button as ButtonType;

use std::time::Instant;

const BALL_RADIUS: f64 = 10.0;

const TIME_SCALING_FACTOR: f64 = 1_f64;

fn main() {
    // whether or not to show the arrow vectors
    let mut show_vectors: bool = false;

    // mouse position
    let mut mouse_x: f64 = 0.0;
    let mut mouse_y: f64 = 0.0;

    // where the mouse was pressed and depressed
    let mut mouse_down_position: Option<Vec2> = None;
    let mut mouse_up_position: Option<Vec2> = None;

    // the last tick; used for calculating dt
    let mut last_tick: Instant = Instant::now();

    // window
    let mut window: PistonWindow = create_window(1600, 800);
    let w = window.size().width;
    let h = window.size().height;

    // balls with gravity
    let mut objects: Vec<Object> = vec![];

    while let Some(event) = window.next() { // program loop
        let mut object_locations: Vec<Vec2> = vec![];

        // calculate dt
        let dt: f64 = last_tick.elapsed().as_secs_f64() * TIME_SCALING_FACTOR;
        last_tick = Instant::now();

        if let Event::Input(input, _) = &event { // handle events
            match *input {
                Move(MouseCursor(pos)) => [mouse_x, mouse_y] = pos, // get mouse x and mouse y
                Button(x) =>
                    if x.button == ButtonType::Mouse(MouseButton::Left) { // mouse left click
                        if x.state == ButtonState::Press {
                            mouse_down_position = Some(Vec2::from_arr([mouse_x, mouse_y]));
                        }

                        if x.state == ButtonState::Release {
                            mouse_up_position = Some(Vec2::from_arr([mouse_x, mouse_y]));
                        }
                    } else if x.button == ButtonType::Keyboard(Key::Backspace) || x.button == ButtonType::Keyboard(Key::Delete) { // clear objects with backspace/delete
                        objects = vec![];
                    } else if x.button == ButtonType::Keyboard(Key::Space) { // space toggle vectors
                        if x.state == ButtonState::Press {
                            show_vectors = !show_vectors
                        }
                    }
                ,
                _ => {} // we ignore all other inputs
            }
        }

        for i in &objects {
            object_locations.push(i.position);
        }

        // loop over all objects and update them
        for i in &mut objects {
            i.update_position(dt, object_locations.clone());
        }

        // check if the user created a object and actually create it
        if let [Some(d), Some(u)] = [mouse_down_position, mouse_up_position] {
            objects.push(Object {
                position: d,
                velocity: (u - d) * 2.0,
                bound_bottom: h - 10.0,
                bound_left: 10.0,
                bound_right: w - 10.0,
                bound_top: 10.0
            });

            // reset
            mouse_down_position = None;
            mouse_up_position = None;
        }

        // drawers for different types of things
        let ellipse_drawer = Ellipse::new([1.0; 4]);
        let line_drawer = Line::new([1.0, 1.0, 1.0, 0.5], 1.0);
        window.draw_2d(&event, |context, graphics, _device| {
            // background
            clear([0.0; 4], graphics);

            // draw each object
            for i in &objects {
                // draw ball
                ellipse_drawer.draw(
                    circle(i.position.x, i.position.y - 10.0, BALL_RADIUS),
                    &context.draw_state,
                    context.transform,
                    graphics
                );

                if show_vectors {
                    // draw the vector arrow if show_vectors
                    line_drawer.draw_arrow(
                        [
                            i.position.x,
                            i.position.y - 10.0,
                            i.position.x + i.velocity.x/3.0,
                            i.position.y - 10.0 + i.velocity.y/3.0 + i.velocity.y.signum()*10.0
                        ],
                        6.0,
                        &context.draw_state,
                        context.transform,
                        graphics
                    );
                }
            };
            if let [Some(x), None] = [mouse_down_position, mouse_up_position] {
                line_drawer.draw_arrow(
                    [
                        x.x,
                        x.y,
                        mouse_x,
                        mouse_y
                    ],
                    6.0,
                    &context.draw_state,
                    context.transform,
                    graphics
                );
            }
        }); // window.draw2d
    } // while let
} // fn main
