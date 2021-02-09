// grade.rs
// CS 487 Pacific Lutheran University
// Author: Cade Lilley - lilleycr@plu.edu
// Takes parameterized sequences for fish project, see key below
// Throws error if values are not 0, 1, 2, or 3
// Throws error if sequences are not of length 30

// Example:
// grade 010101010101010101010101010101 230 notasequence55
// OUTPUT:
// 010101010101010101010101010101 -39.72169735115254
// 230 -1 ERROR: Found length 3, expected length 30:
// notasequence55 0 ERROR: Found length 14, expected length 30: ERROR: Error parsing move sequence. Only 0, 1, 2, and 3 are allowed:

// 0 = "LEFT"
// 1 = "RIGHT"
// 2 = "UP"
// 3 = "DOWN"

use std::env; // Used to read in parameters

fn main() {

    let sequences: Vec<String> = env::args().collect(); // Collects all inputs from args() iter
    //eprintln!("{:?}", sequences);

    let base_state = build_state();
    for i in 1..sequences.len() { // Starts at index 1 to ignore file origin directory parameter
        let mut parsing_error = false;
        let mut cur_state = base_state;
        for char in sequences[i].chars() {
            match char {
                '0' => cur_state = turn(0, cur_state),
                '1' => cur_state = turn(1, cur_state),
                '2' => cur_state = turn(2, cur_state),
                '3' => cur_state = turn(3, cur_state),
                _ => parsing_error = true
            }
        }
        eprint!("{} {} ", sequences[i], cur_state.score);
        if sequences[i].len() != 30 {
            eprint!("ERROR: Found length {}, expected length 30: ", sequences[i].len());
        }
        if parsing_error {
            eprint!("ERROR: Error parsing move sequence. Only 0, 1, 2, and 3 are allowed: ")
        }
        eprintln!(""); // Ends line
    }

}

fn turn(dir: usize, mut state: State) -> State {
    if dir == 0 { // Left
        if state.position.1 != 0 {
            state.position.1 = state.position.1 - 1;
        } else { // Loop around
            state.position.1 = 3;
        }
    } else if dir == 1 { // Right
        state.position.1 = (state.position.1 + 1) % 4; // Loops if out of bounds
    } else if dir == 2 { // Up
        if state.position.0 != 0 {
            state.position.0 = state.position.0 - 1;
        } else { // Loop around
            state.position.0 = 4;
        }
    } else if dir == 3 { // Down
        state.position.0 = (state.position.0 + 1) % 5; // Loops if out of bounds
    }

    let cur_coord_val = state.map[state.position.1][state.position.0];
    state.score = state.score + cur_coord_val;

    // If the current tile value is negative, increase the negativity by 10%
    // Otherwise, deduct 10% of the tile worth and update the map with revised value
    // This is the tile decay function for the game
    if cur_coord_val < 0.0 {
        state.map[state.position.1][state.position.0] = cur_coord_val * 1.1;
    } else {
        state.map[state.position.1][state.position.0] = cur_coord_val * 0.9;
    }

    return state
}

#[derive(Copy, Clone)]
struct State {
    map: [[f64; 5]; 4],
    score: f64,
    position: (usize, usize)
}

/*
 * build_state() when called returns the default game board with default
 * values of step, position, and score.
 * last_move is set to 5 to indicate an impossible move.
*/
fn build_state() -> State {
    let mut map = [[0 as f64; 5]; 4];
    let big_fish = [(0, 2), (1, 0), (2, 3), (3, 2), (4, 3)];
    let dead_fish = [(0, 0), (1, 1), (1, 2), (1, 3), (3, 1), (3, 3), (4, 0), (4, 2)];
    let medium_fish = [(0, 3), (2, 0), (2, 2), (4, 1)];
    let small_fish = [(0, 1), (2, 1), (3, 0)];

    for bf in big_fish.iter() {
        map[bf.1][bf.0] = 5.0;
    }

    for df in dead_fish.iter() {
        map[df.1][df.0] = -2.0;
    }

    for mf in medium_fish.iter() {
        map[mf.1][mf.0] = 3.0;
    }

    for sf in small_fish.iter() {
        map[sf.1][sf.0] = 1.0;
    }
    return State { map: map, score: 0.0, position: (0, 0) }
}
