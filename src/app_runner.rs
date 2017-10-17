use io;
use board;
use game;
use players;
use game_types;
use io::display;
use io::clear_screen;
use board::Board;
use players::Players;
use board_printer::format_board;

const INVALID_VALUE: i32 = -1;
const NUMBER_OF_ROWS: i32 = 3;
const YES: i32 = 1;

pub fn start() {
    let mut board = setup_board();
    let players = setup_players();
    while !game::is_game_over(&board) {
        board = single_turn(board, &players);
    }
    end_of_game(&board);
}

fn setup_board() -> Board {
    io::clear_screen();
    display(io::TITLE);
    board::build_board(NUMBER_OF_ROWS)
}

fn setup_players() -> Vec<Players> {
    let players = io::ask_player_type();
    game_types::create_players(players)
}

fn single_turn(board: Board, players: &[Players]) -> Board {
    show_user_state_of_game(&board);
    let space = select_a_space(&board, players);
    board.place_marker(space)
}

fn end_of_game(board: &Board) {
    clear_screen();
    display(&io::alert_winner(&game::find_winner(board)));
    display(&format_board(board));
    if io::ask_play_again() == YES {
        start()
    }
}

fn select_a_space(board: &Board, players: &[Players]) -> i32 {
    let current_player_marker = game::find_current_player(board);
    let mut space = INVALID_VALUE;
    players
        .iter()
        .filter(|player| players::get_marker(player) == current_player_marker)
        .for_each(|player| space = players::choose_space(player, board));
    space
}

fn show_user_state_of_game(board: &Board) {
    let current_player_marker = game::find_current_player(board);
    clear_screen();
    display(&io::select_space(&current_player_marker));
    display(&format_board(board));
}
