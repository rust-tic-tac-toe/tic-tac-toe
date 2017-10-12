use io::*;
use board::*;
use game::*;

pub fn start() {
    let mut board = setup_board();
    while !is_game_over(&board) {
        board = single_turn(board);
    }
    end_of_game(board);
}

fn setup_board() -> Board {
    // clear_screen();
    display(TITLE);
    let number_of_rows = ask_how_many_rows();
    build_board(number_of_rows)
}

fn single_turn(board: Board) -> Board {
    display(&format_board(&board));
    let space = prompt_for_space(&board);
    board.place_marker(space)
}

fn end_of_game(board: Board) {
    display(&format_board(&board));
    display(&alert_winner(find_winner(&board)));
}

fn prompt_for_space(board: &Board) -> i32 {
    let current_player: String = find_current_player(board);
    ask(&select_space(current_player)) - OFFSET as i32
}
