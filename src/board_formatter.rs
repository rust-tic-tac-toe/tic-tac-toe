use board::Board;
use marker::Marker;
use marker;

pub fn expand_board(board: &Board) -> Vec<String> {
    let spaces = board.get_spaces();
    let number_of_spaces = board.get_size() * board.get_size();
    let mut expanded_board: Vec<String> = vec![" ".to_string(); number_of_spaces as usize];
    for (index, space) in spaces.iter().enumerate() {
        let marker = if is_even(index) {
            marker::inspect(&Marker::X)
        } else {
            marker::inspect(&Marker::O)
        };
        expanded_board[*space as usize] = marker;
    }
    expanded_board
}

fn is_even(index: usize) -> bool {
    index % 2 == 0
}

pub mod tests {
    #[cfg(test)]
    use super::*;
    #[cfg(test)]
    use board::tests::set_up_board;
    #[test]
    fn convert_empty_board() {
        let board = set_up_board(3, vec![]);
        let expanded_board: Vec<String> = vec![
            " ".to_string(),
            " ".to_string(),
            " ".to_string(),
            " ".to_string(),
            " ".to_string(),
            " ".to_string(),
            " ".to_string(),
            " ".to_string(),
            " ".to_string(),
        ];
        assert_eq!(expanded_board, expand_board(&board));
    }

    #[test]
    fn convert_in_progress_board() {
        let board = set_up_board(3, vec![0, 4]);
        let expanded_board: Vec<String> = vec![
            "X".to_string(),
            " ".to_string(),
            " ".to_string(),
            " ".to_string(),
            "O".to_string(),
            " ".to_string(),
            " ".to_string(),
            " ".to_string(),
            " ".to_string(),
        ];
        assert_eq!(expanded_board, expand_board(&board));
    }

    #[test]
    fn convert_full_board() {
        let board = set_up_board(3, vec![0, 4, 8, 2, 6, 7, 1, 3, 5]);
        let expanded_board: Vec<String> = vec![
            "X".to_string(),
            "X".to_string(),
            "O".to_string(),
            "O".to_string(),
            "O".to_string(),
            "X".to_string(),
            "X".to_string(),
            "O".to_string(),
            "X".to_string(),
        ];
        assert_eq!(expanded_board, expand_board(&board));
    }
}
