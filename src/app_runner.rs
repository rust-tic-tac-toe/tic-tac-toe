use io::*;
use board::*;
pub fn start() {
    // clear_screen();
    display(TITLE);
    let number_of_rows = ask(NUMBER_OF_ROWS);
    let mut board = build_board(number_of_rows);
    display(&format_board(&board));
    let space = prompt_for_space();
    board = board.place_marker(space);
    display(&format_board(&board));
}

fn prompt_for_space() -> i32 {
    ask(&select_space("X")) - OFFSET as i32
}
