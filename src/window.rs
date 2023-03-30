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
use crate::managers::road::RoadManager;

/// initializes the window and runs the simulation
///
/// Example
/// ```rust
/// let (roads, intersections) = loaddata::load_data()?;
/// init(roads, intersections); // initializes the window and runs the simulation
/// ```
pub fn init(
    roads: Vec<Road>,
    intersections: Vec<Intersection>,
    logger: &crate::helpers::logger::Logger,
) {
    let opengl = OpenGL::V3_2;

    logger.trace("(window) create window");
    let mut window: PistonWindow = WindowSettings::new("Rust Route", [800, 600])
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();

    let mut state_counter = 0;
    let mut draw_road = false;

    let mut gl = GlGraphics::new(opengl);

    logger.trace("(roadmanager) init road manager");
    let mut road_manager = RoadManager {
        cache: Some(Vec::<Road>::new()),
    };

    logger.trace("(roadmanager) load roads");
    road_manager.load(Some("sample/roads.csv"));

    logger.info("(*) start render loop");
    while let Some(e) = window.next() {
        if let Some(button) = e.press_args() {
            if button == Button::Mouse(MouseButton::Left) {
                if state_counter >= std::usize::MAX - 10 {
                    state_counter = 0;
                }

                state_counter += 1;
            }
        };

        if let Some(button) = e.release_args() {
            if button == Button::Mouse(MouseButton::Left) {
                if state_counter % 2 == 0 {
                    draw_road = true;
                } else {
                    draw_road = false;
                }
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

                for road in road_manager.cache.as_ref().unwrap() {
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
