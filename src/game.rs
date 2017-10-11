use board::Board;
use board::tests::set_up_board;

pub fn find_current_player(board: &Board) -> String {
    let current_player: &str;
    if board.get_spaces().len() % 2 == 0 {
        current_player = "X";
    } else {
        current_player = "O";
    }
    current_player.to_string()
}

pub fn is_game_over(board: &Board) -> bool {
    let max_spaces: i32 = board.get_size() * board.get_size();
    board.get_spaces().len() as i32 == max_spaces
}

pub mod tests {
    use super::*;
    #[cfg(test)]
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


}
