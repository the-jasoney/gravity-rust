mod window;
mod events;
mod draw;
mod sim;
mod util;

extern crate piston_window;
extern crate lazy_static;
extern crate find_folder;
extern crate fps_counter;

use crate::window::create_window;
//use crate::events::handle_events;
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

const WIDTH: u32 = 1600;
const HEIGHT: u32 = 800;

const FLOOR_Y: f64 = HEIGHT as f64 - 10.0;

fn main() {
    let mut counter = fps_counter::FPSCounter::new();

    let mut show_vectors: bool = false;

    let mut mouse_x: f64 = 0.0;
    let mut mouse_y: f64 = 0.0;

    let mut mouse_down_position: Option<Vec2> = None;
    let mut mouse_up_position: Option<Vec2> = None;

    let mut last_tick: Instant = Instant::now();

    let mut window: PistonWindow = create_window(WIDTH, HEIGHT);

    let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
    let mut glyphs = window.load_font(assets.join("Roboto-Medium.ttf")).unwrap();

    /*let mut gravity_object = Object {
        position: vec2!(WIDTH/2, 0),
        velocity: vec2!(0.0, 0),
        floor_y: FLOOR_Y,
        r: BALL_RADIUS,
        ..Object::default()
    };*/

    let mut objects: Vec<Object> = vec![];

    let ellipse_drawer = Ellipse::new([1.0; 4]);
    let line_drawer = Line::new([1.0, 1.0, 1.0, 0.5], 1.0);
    let floor_drawer = Line::new([1.0; 4], 1.0);
    let text_drawer = Text::new_color([1.0; 4], 15);

    while let Some(event) = window.next() {
        let fps = counter.tick();
        //println!("{}", fps);

        if let Event::Input(input, _) = &event {
            [mouse_x, mouse_y] = match *input {
                Move(
                    MouseCursor(
                        pos
                    )
                ) => pos,
                _ => [mouse_x, mouse_y]
            };

            match *input {
                Button(x) => {
                    if x.button == ButtonType::Mouse(MouseButton::Left) {
                        if x.state == ButtonState::Press {
                            mouse_down_position = Some(Vec2::from_arr([mouse_x, mouse_y]));
                        }

                        if x.state == ButtonState::Release {
                            mouse_up_position = Some(Vec2::from_arr([mouse_x, mouse_y]));
                        }
                    } else if x.button == ButtonType::Keyboard(Key::Backspace) {
                        objects = vec![];
                    } else if x.button == ButtonType::Keyboard(Key::Space) {
                        if x.state == ButtonState::Press {
                            show_vectors = !show_vectors
                        }
                    }
                },
                _ => {}
            };

            //println!("{:#?}", *input);
        }

        let dt: f64 = last_tick.elapsed().as_secs_f64();
        last_tick = Instant::now();

        for i in &mut objects {
            i.update_position(dt)
        }

        //let visible_x = gravity_object.position.x % WIDTH as f64;
        //let visible_y = (gravity_object.position.y % HEIGHT as f64) - BALL_RADIUS;

        if let [Some(d), Some(u)] = [mouse_down_position, mouse_up_position] {
            objects.push(Object {
                position: d,
                velocity: u - d,
                floor_y: FLOOR_Y,
                r: 10.0
            });

            println!("start: {}, end: {}", d, u);

            mouse_down_position = None;
            mouse_up_position = None;
        }

        window.draw_2d(&event, |context, graphics, _device| {
            clear([0.0; 4], graphics);

            for i in &objects {
                ellipse_drawer.draw(
                    circle(i.position.x, i.position.y - 10.0, BALL_RADIUS),
                    &context.draw_state,
                    context.transform,
                    graphics
                );

                if show_vectors {
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
            }

            text_drawer.draw_pos(
                ("fps: ".to_owned() + fps.to_string().as_str()).as_str(),
                [(WIDTH/2) as f64, 10.0],
                &mut glyphs,
                &context.draw_state,
                context.transform,
                graphics
            ).unwrap();

            floor_drawer.draw(
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
