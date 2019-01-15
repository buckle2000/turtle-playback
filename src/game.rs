use rl::Vector2;
use rl::Color;

const FPS: i32 = 120;
const TRANSPARENT_WHITE: Color = Color {
    a: 10,
    ..rl::WHITE
};

#[derive(Clone, Debug)]
pub struct TurtleState {
    pub time: f32,
    /// in pixels
    pub pos: Vector2, 
    /// in radians
    pub rot: f32,
    pub is_drawing: bool,
}

pub struct GameState {
    pub spawn_interval: f32,
    pub move_speed: f32,
    pub rotate_speed: f32,
    pub turtles: Vec<TurtleState>,
    pub running: bool,
}

pub fn rotate_vec2(angle: f32) -> Vector2 {
    Vector2 {
        x: angle.cos(),
        y: angle.sin(),
    }
}

use std::ops::{Add, Mul};

/// linear interpolate
pub fn interp<T>(a: T, b: T, coefficient: f32) -> T
    where T: Mul<f32, Output=T> + Add<Output=T>
{
    a * (1. - coefficient) + b * coefficient
}

pub fn interpolate_state(states: &Vec<TurtleState>, time: f32) -> Option<TurtleState> {
    let mut i = 0;
    while i < states.len() {
        let state_after = &states[i];
        if state_after.time > time {
            if i == 0 {
                return None
            }
            let state_before = &states[i-1];
            let interp_co: f32 = // coefficient
                (time - state_before.time) /
                (state_after.time - state_before.time);
            return Some(TurtleState {
                time,
                pos: interp(state_before.pos, state_after.pos, interp_co),
                rot: interp(state_before.rot, state_after.rot, interp_co),
                is_drawing: state_before.is_drawing
            })
        }
        i += 1;
    }
    return None
}

/// Draw turtle
/// return: if turtle is at the end of script
pub fn draw_turtle(states: &Vec<TurtleState>, time: f32) -> bool {
    if let Some(current_state) = interpolate_state(&states, time) {
        {
            let p = current_state.pos;
            let a = current_state.rot;
            if current_state.is_drawing {
                rl::draw_circle_v(p, 3., rl::BLACK);
                rl::draw_triangle(rotate_vec2(a + 140.) * 5. + p, rotate_vec2(a + 0.) * 5. + p, rotate_vec2(a - 140.) * 5. + p, rl::VIOLET);
            } else {
                rl::draw_triangle_lines(rotate_vec2(a + 140.) * 5. + p, rotate_vec2(a + 0.) * 5. + p, rotate_vec2(a - 140.) * 5. + p, rl::VIOLET);
            }
        }
        true
    } else {
        false
    }
}

pub fn run_game(game_state: &GameState) {
    rl::set_trace_log(rl::LOG_WARNING);
    rl::init_window(800, 450, "raylib [core] example - basic window");
    rl::set_target_fps(FPS);

    let states = &game_state.turtles;
    let screen_center = Vector2::new(rl::get_screen_width() as f32 / 2., rl::get_screen_height() as f32 / 2.);
    let camera = rl::Camera2D {
        target: Vector2::zero(),
        offset: screen_center,
        rotation: 0.,
        zoom: 1.,
    };

    // all the turtles
    let mut time_offsets = vec![];

    {
        let mut i = 1;
        while let Some(_) = interpolate_state(&states, game_state.spawn_interval * i as f32) {
            time_offsets.push(game_state.spawn_interval * i as f32);
            i += 1;
        }
    }
    
    let mut next_spawn_time = 0.;

    while !rl::window_should_close() && game_state.running {
        let dt = rl::get_frame_time();

        if next_spawn_time <= 0. {
            next_spawn_time += game_state.spawn_interval;
            time_offsets.push(0.);
        }

        rl::begin_drawing();
        // rl::clear_background(TRANSPARENT_WHITE);
        rl::draw_rectangle(0, 0, rl::get_screen_width(), rl::get_screen_height(), TRANSPARENT_WHITE);
        rl::begin_mode_2d(camera);

        time_offsets = time_offsets.into_iter().filter(|time| draw_turtle(&states, *time)).collect();

        rl::end_mode_2d();
        rl::end_drawing();

        for time_offset in time_offsets.iter_mut() {
            *time_offset += dt;
        }

        next_spawn_time -= dt;
    }
    rl::close_window();
}