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

use crate::loaddata::{Intersection, Road};

pub fn init(roads: Vec<Road>, intersections: Vec<Intersection>) {
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new("Rust Route", [800, 600])
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();

    let mut gl = GlGraphics::new(opengl);

    while let Some(e) = window.next() {
        if let Some(r) = e.render_args() {
            gl.draw(r.viewport(), |c, gl| {
                clear([1.0; 4], gl);
                for road in &roads {
                    line(
                        [0.0, 0.0, 0.0, 1.0],
                        1.0,
                        road.get_points(),
                        c.transform,
                        gl,
                    );
                }
                for intersection in &intersections {
                    ellipse(
                        [0.0, 0.0, 0.0, 1.0],
                        [intersection.lon - 2.0, intersection.lat - 2.0, 4.0, 4.0],
                        c.transform,
                        gl,
                    );
                }
            });
        }
    }
}
