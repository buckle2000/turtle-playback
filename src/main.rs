extern crate raylib_rs as rl;

mod game;

use crate::game::*;

use rl::Vector2;

pub fn load_states(filename: &str, game_state: &mut GameState) {
    let data = std::fs::read_to_string(filename).unwrap();

    let initial_state = TurtleState {
        time: 0.,
        pos: Vector2::zero(),
        rot: 0.,
        is_drawing: true,
    };
    
    let states = &mut game_state.turtles;
    states.push(initial_state);

    for lines in data.split_terminator("\n") {
        let tokens: Vec<&str> = lines.split(" ").collect();
        assert!(tokens.len() >= 1);

        let last_state: TurtleState = states.last().unwrap().clone();

        let new_state = match tokens[0] {
            "pu" => TurtleState {
                is_drawing: false,
                ..last_state
            },
            "pd" => TurtleState {
                is_drawing: true,
                ..last_state
            },
            "fd" => {
                let displacement: f32 = tokens[1].parse().unwrap();
                TurtleState {
                    time: last_state.time + displacement.abs() / game_state.move_speed,
                    pos: last_state.pos + rotate_vec2(last_state.rot) * displacement,
                    ..last_state
                }
            },
            "rt" => {
                let angle_in_degrees: f32 = tokens[1].parse().unwrap();
                TurtleState {
                    time: last_state.time + angle_in_degrees.abs() / game_state.rotate_speed,
                    rot: last_state.rot + angle_in_degrees.to_radians(),
                    ..last_state
                }
            },
            x => panic!("Invalid instruction: {}", x)
        };
        states.push(new_state);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        let script_filename = &args[1];

        // change these numbers and see what will happen
        let mut game_state = GameState {
            move_speed: 80.,
            rotate_speed: 10000.,
            spawn_interval: 3.,
            turtles: Vec::new(),
            running: true,
        };

        load_states(script_filename, &mut game_state);
        if cfg!(debug_assertions) { dbg!(&game_state.turtles); }

        run_game(&game_state);
    } else {
        println!("Usage: {} <SCRIPT>", args[0]);
    }
}
