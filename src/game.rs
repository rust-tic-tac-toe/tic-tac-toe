use lines;
use marker;
use marker::Marker;
use board::Board;

pub fn find_current_player(board: &Board) -> Marker {
    let current_player = if board.get_spaces().len() % 2 == 0 {
        Marker::X
    } else {
        Marker::O
    };
    current_player
}

pub fn is_game_over(board: &Board) -> bool {
    is_game_tied(board) || is_game_won(board)
}

pub fn is_game_tied(board: &Board) -> bool {
    !is_game_won(board) && board.get_available_spaces().is_empty()
}

fn is_game_won(board: &Board) -> bool {
    is_game_won_by(board, &Marker::X) || is_game_won_by(board, &Marker::O)
}

pub fn is_game_won_by(board: &Board, player: &Marker) -> bool {
    let winning_scenarios = lines::find_all_lines(board);
    winning_scenarios
        .iter()
        .any(|line| is_line_won_by(line, player))
}

fn is_line_won_by(line: &[String], player: &Marker) -> bool {
    line.iter().all(|space| &space.to_string() == &marker::inspect(&player))
}

pub fn find_winner(board: &Board) -> String {
    let winner = if is_game_won_by(board, &Marker::X) {
        "X"
    } else if is_game_won_by(board, &Marker::O) {
        "O"
    } else {
        "Nobody"
    };
    winner.to_string()
}

pub mod tests {
    #[cfg(test)]
    use super::*;
    #[cfg(test)]
    use board::tests::set_up_board;
    #[test]
    fn x_is_current_player_at_start_of_game() {
        let board: Board = set_up_board(3, vec![]);
        assert_eq!(Marker::X, find_current_player(&board));
    }

    #[test]
    fn o_is_current_player_after_one_move() {
        let board: Board = set_up_board(3, vec![0]);
        assert_eq!(Marker::O, find_current_player(&board));
    }

    #[test]
    fn game_not_over_when_board_is_empty() {
        let board: Board = set_up_board(3, vec![]);
        assert!(!is_game_over(&board));
    }

    #[test]
    fn game_is_over_when_board_is_full() {
        let board: Board = set_up_board(3, vec![0, 4, 8, 2, 6, 7, 1, 3, 5]);
        assert!(is_game_over(&board));
    }

    #[test]
    fn check_if_tied_game_is_won() {
        let board = set_up_board(3, vec![0, 4, 8, 2, 6, 7, 1, 3, 5]);
        assert!(!is_game_won(&board));
    }

    #[test]
    fn check_if_game_won_by_x_is_won() {
        let board = set_up_board(3, vec![0, 4, 8, 2, 6, 3, 7]);
        assert!(is_game_won(&board));
    }

    #[test]
    fn check_if_game_won_by_o_is_won() {
        let board = set_up_board(3, vec![0, 8, 4, 7, 2, 6]);
        assert!(is_game_won(&board));
    }

    #[test]
    fn an_empty_game_is_not_won() {
        let board = set_up_board(3, vec![]);
        assert!(!is_game_won(&board));
    }

    #[test]
    fn an_empty_game_is_not_tied() {
        let board = set_up_board(3, vec![]);
        assert!(!is_game_tied(&board));
    }

    #[test]
    fn a_won_game_is_not_tied() {
        let board = set_up_board(3, vec![0, 8, 4, 7, 2, 6]);
        assert!(!is_game_tied(&board));
    }

    #[test]
    fn a_won_game_with_a_full_board_is_not_tied() {
        let board = set_up_board(3, vec![0, 3, 1, 4, 6, 7, 5, 8, 2]);
        assert!(!is_game_tied(&board));
    }

    #[test]
    fn a_tied_game_is_tied() {
        let board = set_up_board(3, vec![0, 4, 8, 2, 6, 7, 1, 3, 5]);
        assert!(is_game_tied(&board));
    }

    #[test]
    fn check_line_won_by_x() {
        let line: Vec<String> = vec!["X".to_string(), "X".to_string(), "X".to_string()];
        assert!(is_line_won_by(&line, &Marker::X));
    }

    #[test]
    fn check_row_not_won_by_o() {
        let line: Vec<String> = vec!["O".to_string(), " ".to_string(), "X".to_string()];
        assert!(!is_line_won_by(&line, &Marker::O));
    }

    #[test]
    fn find_winner_when_nobody_has_won() {
        let board = set_up_board(3, vec![0, 4, 8, 2, 6, 7, 1, 3, 5]);
        assert_eq!("Nobody".to_string(), find_winner(&board));
    }

    #[test]
    fn find_winner_when_x_has_won() {
        let board = set_up_board(3, vec![0, 4, 8, 2, 6, 3, 7]);
        assert_eq!("X".to_string(), find_winner(&board));
    }

    #[test]
    fn find_winner_when_o_has_won() {
        let board = set_up_board(3, vec![0, 8, 4, 7, 2, 6]);
        assert_eq!("O".to_string(), find_winner(&board));
    }
}
