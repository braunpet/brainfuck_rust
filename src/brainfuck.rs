struct State {
    data: [u8; 30000],
    memptr: usize,
    prgptr: usize,
    program: Vec<char>,
}
enum Direction {
    Left,
    Right,
}
pub fn run(prg: String) {
    let mut state = State {
        data: [0; 30000],
        memptr: 0,
        prgptr: 0,
        program: prg.chars().collect(),
    };

    while state.prgptr < state.program.len() {
        execute_statement(&mut state);
    }
}

fn execute_statement(state: &mut State) {
    match state.program[state.prgptr] {
        '>' => state.memptr += 1,
        '<' => state.memptr -= 1,
        '+' => state.data[state.memptr] += 1,
        '-' => state.data[state.memptr] -= 1,
        '.' => print_character(state),
        '[' => jump_forward(state),
        ']' => jump_backward(state),
        _ => println!("Unknown command"),
    }
    state.prgptr += 1;
}

fn print_character(state: &mut State) {
    print!("{}", state.data[state.memptr] as char);
}

fn jump_forward(state: &mut State) {
    if state.data[state.memptr] == 0 {
        search_for_stmt_in_direction(state, ']', &Direction::Right);
    }
}

fn jump_backward(state: &mut State) {
    if state.data[state.memptr] != 0 {
        search_for_stmt_in_direction(state, '[', &Direction::Left);
    }
}

fn search_for_stmt_in_direction(state: &mut State, stmt: char, direction: &Direction) {
    while state.program[state.prgptr] as char != stmt {
        match direction {
            Direction::Left => state.prgptr -= 1,
            Direction::Right => state.prgptr += 1,
        }
    }
}
