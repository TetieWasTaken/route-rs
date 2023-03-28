extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate piston_window;

/* PSEUDO

    - Create a window
    - Create a loop that runs until the window is closed
    - In the loop, draw the roads and intersections
    - Once the simulation is done, close the window

*/

use graphics::*;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::*;
use piston_window::*;

use crate::managers::intersection::Intersection;
use crate::managers::road::Road;

/// initializes the window and runs the simulation
///
/// Example
/// ```rust
/// let (roads, intersections) = loaddata::load_data()?;
/// init(roads, intersections); // initializes the window and runs the simulation
/// ```
pub fn init(roads: Vec<Road>, intersections: Vec<Intersection>) {
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new("Rust Route", [800, 600])
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();

    let mut draw_road = false;

    let mut gl = GlGraphics::new(opengl);

    while let Some(e) = window.next() {
        if let Some(button) = e.press_args() {
            if button == Button::Mouse(MouseButton::Left) {
                draw_road = !draw_road;
            }
        };

        if draw_road {
            if let Some(pos) = e.mouse_cursor_args() {
                println!("Mouse position: {:?}", pos);
            }
        }

        if let Some(r) = e.render_args() {
            gl.draw(r.viewport(), |c, gl| {
                clear([0.1137, 0.1098, 0.0902, 1.0], gl);

                let center = c
                    .transform
                    .trans(r.window_size[0] / 2.0, r.window_size[1] / 2.0);
                let scale = center.scale(10.0, 10.0);

                for road in &roads {
                    let color;

                    match road.road_type.as_str() {
                        "asphalt" => color = [0.3529, 0.3529, 0.3529, 1.0],
                        "dirt" => color = [0.5, 0.5, 0.5, 1.0],
                        "gravel" => color = [0.8, 0.8, 0.8, 1.0],
                        _ => color = [0.0, 0.0, 0.0, 1.0],
                    }

                    line(color, 1.0, road.get_points(), scale, gl);
                }
                for intersection in &intersections {
                    ellipse(
                        [0.0, 0.0, 1.0, 1.0],
                        [intersection.lon - 2.0, intersection.lat - 2.0, 4.0, 4.0],
                        scale.trans(intersection.lon, -intersection.lat),
                        gl,
                    );
                }
            });
        }
    }
}
