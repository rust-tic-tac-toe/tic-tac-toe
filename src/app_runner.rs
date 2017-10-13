use io::*;
use board::*;
use game::*;
use players::*;
use game_types::*;

const INVALID_VALUE: i32 = -1;
const NUMBER_OF_ROWS: i32 = 3;

pub fn start() {
    let mut board = setup_board();
    let players = setup_players();
    while !is_game_over(&board) {
        board = single_turn(board, &players);
    }
    end_of_game(&board);
}

fn setup_board() -> Board {
    clear_screen();
    display(TITLE);
    build_board(NUMBER_OF_ROWS)
}

fn setup_players() -> Vec<Players> {
    let players = ask_player_type();
    create_players(players)
}

fn single_turn(board: Board, players: &[Players]) -> Board {
    display(&format_board(&board));
    let space = find_space(&board, players);
    board.place_marker(space)
}

fn end_of_game(board: &Board) {
    display(&format_board(board));
    display(&alert_winner(&find_winner(board)));
}

fn find_space(board: &Board, players: &[Players]) -> i32 {
    let current_player_marker: String = find_current_player(board);
    let mut space: i32 = INVALID_VALUE;
    for player in players.iter() {
        if current_player_marker == get_marker(player) {
            space = choose_space(player, board)
        }
    }
    space
}
