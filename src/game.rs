use board::*;
use lines::*;
use board_formatter::expand_board;

pub fn find_current_player(board: &Board) -> String {
    let current_player = if board.get_spaces().len() % 2 == 0 {
        "X"
    } else {
        "O"
    };
    current_player.to_string()
}

pub fn is_game_over(board: &Board) -> bool {
    is_game_tied(board) || is_game_won(board)
}

pub fn is_game_tied(board: &Board) -> bool {
    !is_game_won(board) && board.get_available_spaces().is_empty()
}

fn is_game_won(board: &Board) -> bool {
    is_game_won_by(board, "X") || is_game_won_by(board, "O")
}

pub fn is_game_won_by(board: &Board, player: &str) -> bool {
    let winning_scenarios = find_winning_scenarios(board);
    winning_scenarios
        .iter()
        .any(|line| is_line_won_by(line, player))
}

fn is_line_won_by(line: &[String], player: &str) -> bool {
    line.iter().all(|space| space == player)
}

fn find_winning_scenarios(board: &Board) -> Vec<Vec<String>> {
    let mut winning_scenarios: Vec<Vec<String>> = Vec::new();
    let mut rows = split_board_into_rows(&expand_board(board), board.get_size().abs());
    let mut columns = find_columns(&rows);
    let left = find_left_diagonal(&rows);
    let right = find_right_diagonal(&rows);
    winning_scenarios.append(&mut rows);
    winning_scenarios.append(&mut columns);
    winning_scenarios.push(left);
    winning_scenarios.push(right);
    winning_scenarios
}

pub fn find_winner(board: &Board) -> String {
    if is_game_won_by(board, "X") {
        "X".to_string()
    } else if is_game_won_by(board, "O") {
        "O".to_string()
    } else {
        "Nobody".to_string()
    }
}

pub mod tests {
    #[cfg(test)]
    use super::*;
    #[cfg(test)]
    use board::tests::set_up_board;
    #[test]
    fn x_is_current_player_at_start_of_game() {
        let board: Board = set_up_board(3, vec![]);
        assert_eq!("X", find_current_player(&board));
    }

    #[test]
    fn o_is_current_player_after_one_move() {
        let board: Board = set_up_board(3, vec![0]);
        assert_eq!("O", find_current_player(&board));
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
    fn finds_winning_scenarios() {
        let board = set_up_board(3, vec![0, 4, 8, 2, 6, 7, 1, 3, 5]);
        let winning_scenarios: Vec<Vec<String>> = vec![
            vec!["X".to_string(), "X".to_string(), "O".to_string()],
            vec!["O".to_string(), "O".to_string(), "X".to_string()],
            vec!["X".to_string(), "O".to_string(), "X".to_string()],
            vec!["X".to_string(), "O".to_string(), "X".to_string()],
            vec!["X".to_string(), "O".to_string(), "O".to_string()],
            vec!["O".to_string(), "X".to_string(), "X".to_string()],
            vec!["X".to_string(), "O".to_string(), "X".to_string()],
            vec!["O".to_string(), "O".to_string(), "X".to_string()],
        ];
        assert_eq!(winning_scenarios, find_winning_scenarios(&board));
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
        assert!(is_line_won_by(&line, "X"));
    }

    #[test]
    fn check_row_not_won_by_o() {
        let line: Vec<String> = vec!["O".to_string(), " ".to_string(), "X".to_string()];
        assert!(!is_line_won_by(&line, "O"));
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
