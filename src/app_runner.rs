use io::*;
use board::*;
use game::*;

pub fn start() {
    let mut board = setup_board();
    board = single_turn(board);
    display(&format_board(&board));
}

fn setup_board() -> Board {
    // clear_screen();
    display(TITLE);
    let number_of_rows = ask(NUMBER_OF_ROWS);
    build_board(number_of_rows)
}

fn single_turn(board: Board) -> Board {
    display(&format_board(&board));
    let space = prompt_for_space(&board);
    board.place_marker(space)
}

fn prompt_for_space(board: &Board) -> i32 {
    let current_player: String = find_current_player(board);
    ask(&select_space(current_player)) - OFFSET as i32
}


