mod window;
mod events;
mod draw;
mod sim;

extern crate piston_window;
extern crate lazy_static;
extern crate find_folder;

use crate::window::create_window;
//use crate::events::handle_events;
use crate::sim::object::Object;
use sim::vec2::Vec2;
use sim::vec2;

use piston_window::ellipse::circle;
use piston_window::*;

use std::time::Instant;

const BALL_RADIUS: f64 = 10.0;

const WIDTH: u32 = 1600;
const HEIGHT: u32 = 800;

const FLOOR_Y: f64 = HEIGHT as f64 - 10.0;

fn main() {

    let mut last_tick: Instant = Instant::now();

    let mut window: PistonWindow = create_window(WIDTH, HEIGHT);

    let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
    let mut glyphs = window.load_font(assets.join("Roboto-Medium.ttf")).unwrap();

    let mut gravity_object = Object {
        position: vec2!(0.0, 0),
        velocity: vec2!(1000.0, 0),
        floor_y: FLOOR_Y,
        r: BALL_RADIUS,
        ..Object::default()
    };

    let ellipse_drawer = Ellipse::new([1.0; 4]);
    let line_drawer = Line::new([1.0; 4], 1.0);
    let text_drawer = Text::new_color([1.0; 4], 15);

    while let Some(event) = window.next() {
        let dt: f64 = last_tick.elapsed().as_secs_f64();
        last_tick = Instant::now();

        gravity_object.update_position(dt);
        //println!("{}", gravity_object);

        let visible_x = gravity_object.position.x % WIDTH as f64;
        let visible_y = (gravity_object.position.y % HEIGHT as f64) - BALL_RADIUS;

        window.draw_2d(&event, |context, graphics, _device| {
            clear([0.0; 4], graphics);

            ellipse_drawer.draw(
                circle(visible_x, visible_y, BALL_RADIUS),
                &DrawState {scissor: None, stencil: None, blend: None},
                context.transform,
                graphics
            );

            line_drawer.draw_arrow(
                [
                    visible_x,
                    visible_y,
                    visible_x + gravity_object.velocity.x/3.0,
                    visible_y + gravity_object.velocity.y/3.0 + gravity_object.velocity.y.signum()*10.0,
                ],
                6.0,
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

            for i in 0..=HEIGHT/100 {
                text_drawer.draw_pos(
                i.to_string().as_str(),
                    [10.0, i as f64 * 100.0],
                    &mut glyphs,
                    &context.draw_state,
                    context.transform,
                    graphics
                ).unwrap();
            }
        });
    }
}
