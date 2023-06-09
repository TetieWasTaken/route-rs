use graphics::*;
use opengl_graphics::{GlGraphics, GlyphCache, OpenGL};
use piston::input::*;
use piston_window::*;
use sdl2_window::Sdl2Window as Window;

use std::sync::{Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::constants::colors::*;
use crate::get_history_manager;
use crate::managers::intersection::Intersection;
use crate::managers::road::Road;

#[derive(PartialEq)]
enum States {
    DrawRoad,
    DrawIntersection,
    Destroy,
}

/// initializes the window and runs the simulation
///
/// Example
/// ```rust
/// init(); // initializes the window and runs the simulation
/// ```
pub fn init() {
    let opengl = OpenGL::V3_2;

    let logger = crate::get_logger();

    logger.trace("(window) create window");
    let mut window: Window = WindowSettings::new("Rust Route", [800, 600])
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();

    let mut state_counter = 1;
    let mut state = States::DrawRoad;
    let mut draw_road = false;
    let mut draw_intersection = false;
    let mut start_point: Option<[f64; 2]> = None;
    let mut road_to_draw: [f64; 4] = [0.0, 0.0, 0.0, 0.0];
    let mut intersection_to_draw: [f64; 2] = [0.0, 0.0];
    let mut latest_mouse_pos: [f64; 2] = [0.0, 0.0];

    let font = "assets/FiraSans-Regular.ttf";
    let mut glyphs = GlyphCache::new(font, (), TextureSettings::new()).unwrap();

    let mut gl = GlGraphics::new(opengl);

    logger.trace("(roadmanager) init road manager");
    let mut road_manager = crate::get_road_manager().write().unwrap();

    logger.trace("(roadmanager) load roads");
    road_manager.load(Some("sample/roads.csv"));

    logger.trace("(intersectionmanager) init intersection manager");
    let mut intersection_manager = crate::get_intersection_manager().lock().unwrap();

    logger.trace("(intersectionmanager) load intersections");
    intersection_manager.load(Some("sample/intersections.csv"));

    logger.info("(*) start render loop");

    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        if let Some(button) = e.press_args() {
            if button == Button::Mouse(MouseButton::Left) {
                if state_counter >= std::usize::MAX - 10 {
                    state_counter = 1;
                }

                state_counter += 1;
            }
        };

        if let Some(pos) = e.mouse_cursor_args() {
            latest_mouse_pos = pos;
        }

        if let Some(button) = e.release_args() {
            if button == Button::Mouse(MouseButton::Left) {
                match state {
                    States::DrawRoad => {
                        if state_counter % 2 == 0 {
                            draw_road = true;
                            start_point = e.mouse_cursor_args();
                        } else {
                            road_manager.create(Road {
                                _id: None,
                                name: "test".to_string(),
                                start_lat: road_to_draw[0],
                                start_lon: road_to_draw[1],
                                stop_lat: road_to_draw[2],
                                stop_lon: road_to_draw[3],
                                lane_count: 1.0,
                                speed_limit: 50.0,
                                road_type: "asphalt".to_string(),
                            });

                            draw_road = false;
                            start_point = None;
                        }
                    }
                    States::DrawIntersection => {
                        if state_counter % 2 == 0 {
                            draw_intersection = true;
                        } else {
                            intersection_manager.create(Intersection {
                                _id: None,
                                lat: intersection_to_draw[0],
                                lon: intersection_to_draw[1],
                                traffic_lights: false,
                            });

                            draw_intersection = false;
                        }
                    }
                    States::Destroy => {
                        let mut roads_to_destroy = vec![];

                        for road in road_manager.cache.as_ref().unwrap().iter() {
                            let segments = road.segment(10.0);

                            for segment in segments {
                                let dx = segment.0 - latest_mouse_pos[0];
                                let dy = segment.1 - latest_mouse_pos[1];
                                let distance = (dx.powi(2) + dy.powi(2)).sqrt();

                                if distance <= 5.0 {
                                    roads_to_destroy.push(road._id.unwrap());
                                    break;
                                }
                            }
                        }

                        for road_id in roads_to_destroy {
                            road_manager.destroy(road_id);
                        }

                        let mut intersections_to_destroy = vec![];

                        for intersection in intersection_manager.cache.as_ref().unwrap().iter() {
                            let dx = intersection.lat - latest_mouse_pos[0];
                            let dy = intersection.lon - latest_mouse_pos[1];
                            let distance = (dx.powi(2) + dy.powi(2)).sqrt();

                            if distance <= 6.0 {
                                intersections_to_destroy.push(intersection._id.unwrap());
                                break;
                            }
                        }

                        for intersection_id in intersections_to_destroy {
                            intersection_manager.destroy(intersection_id);
                        }
                    }
                    _ => panic!("invalid state"),
                }
            }
        };

        if draw_road {
            if let Some(pos) = e.mouse_cursor_args() {
                if !start_point.is_some() {
                    start_point = Some(pos);
                }
                road_to_draw = [
                    start_point.unwrap()[0],
                    start_point.unwrap()[1],
                    pos[0],
                    pos[1],
                ];
            }
        } else {
            road_to_draw = [0.0, 0.0, 0.0, 0.0];
        }

        if draw_intersection {
            if let Some(pos) = e.mouse_cursor_args() {
                intersection_to_draw = pos;
            }
        } else {
            intersection_to_draw = [0.0, 0.0];
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            if key == Key::Space {
                state_counter = 1;
                draw_road = false;
                draw_intersection = false;
                start_point = None;

                match state {
                    States::DrawRoad => {
                        state = States::DrawIntersection;
                    }
                    States::DrawIntersection => {
                        state = States::Destroy;
                    }
                    States::Destroy => {
                        state = States::DrawRoad;
                    }
                }
            }

            if key == Key::Z {
                println!("undo");
                get_history_manager().lock().unwrap().undo();
                // FIXME: UNDO DEADLOCK
                // TODO: UNDO
            }
        }

        if let Some(r) = e.render_args() {
            gl.draw(r.viewport(), |c, gl| {
                clear(COLOR_BACKGROUND, gl);

                if road_to_draw != [0.0, 0.0, 0.0, 0.0] {
                    line(COLOR_ASPHALT, 5.0, road_to_draw, c.transform, gl);
                }

                if intersection_to_draw != [0.0, 0.0] {
                    ellipse(
                        COLOR_INTERSECTION,
                        [
                            intersection_to_draw[0] - 6.0,
                            intersection_to_draw[1] - 6.0,
                            12.0,
                            12.0,
                        ],
                        c.transform,
                        gl,
                    );
                }

                for road in road_manager.cache.as_ref().unwrap() {
                    let color;

                    match road.road_type.as_str() {
                        "asphalt" => color = COLOR_ASPHALT,
                        "dirt" => color = COLOR_DIRT,
                        "gravel" => color = COLOR_GRAVEL,
                        _ => color = COLOR_SOLID_BLACK,
                    }

                    line(color, 5.0, road.get_points(), c.transform, gl);
                }
                for intersection in intersection_manager.cache.as_ref().unwrap() {
                    ellipse(
                        COLOR_INTERSECTION,
                        [intersection.lat - 6.0, intersection.lon - 6.0, 12.0, 12.0],
                        c.transform,
                        gl,
                    );
                }

                let state_text = match state {
                    States::DrawRoad => "STATE: Road",
                    States::DrawIntersection => "STATE: Intersection",
                    States::Destroy => "STATE: Destroy",
                };

                let mut text = graphics::Text::new(32);

                let text_width = glyphs.width(32, state_text).unwrap_or(0.0) as f64;
                let text_height = glyphs.character(32, 'M').unwrap().advance_height() as f64;

                let transform = c.transform.trans(
                    c.viewport.unwrap().window_size[0] - text_width - 10.0,
                    c.viewport.unwrap().window_size[1] - text_height - 10.0,
                );

                text.color = [1.0, 1.0, 1.0, 1.0];
                text.draw(state_text, &mut glyphs, &c.draw_state, transform, gl)
                    .unwrap();
            });
        }
    }
}
