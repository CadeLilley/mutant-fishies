// Hard code in sequences to get scores
// 0 = "LEFT"
// 1 = "RIGHT"
// 2 = "UP"
// 3 = "DOWN"

/*
"332323232301003232323232101100",
"332300103232112300323232112323",
"332323230123003232103232103211",
"332323232301001032323232103211",
"330123232323001032323232103211",
"332301230032112323003232323211",
"330123230123230032321032323211",
"332323003232323210110123230032",
"332323003232110032321123230032",
"330032323232110123232323010032",
"330123001123232300323232321032",
"330123232301230032323232101032",
"330101012323232300323232321032",
"330032112323230123001032323232",
"332301232301230010321032323232",
"330123232301230100103232323232",
"330032103232323211012323230123",
"330032321101232323003232321123"
*/

fn main() {
    let sequences = ["332323232301003232323232101100", "330032321101232323003232321123"];

    let base_state = build_state();
    for i in 0..sequences.len() {
        let mut cur_state = base_state;
        print!("{}   ", sequences[i]);
        for char in sequences[i].chars() {
            match char {
                '0' => cur_state = turn(0, cur_state),
                '1' => cur_state = turn(1, cur_state),
                '2' => cur_state = turn(2, cur_state),
                '3' => cur_state = turn(3, cur_state),
                _ => eprintln!("Move not in bounds. Only 0, 1, 2, and 3 are allowed")
            }
        }
        println!("{}", cur_state.score);
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
