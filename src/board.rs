#[derive(Default)]
pub struct Board {
    size: i32,
    spaces: Vec<i32>,
}

pub fn build_board(size: i32) -> Board {
    Board {
        size,
        spaces: Vec::new(),
    }
}

impl Board {

    pub fn get_size(&self) -> &i32 {
        &self.size
    }

    pub fn get_spaces(&self) -> &Vec<i32> {
        &self.spaces
    }

    pub fn place_marker(self, space: i32) -> Board {
        if self.is_move_valid(&space) {
            self.create_new_board_with_move(space)
        } else {
            self
        }
    }

    fn is_move_valid(&self, space: &i32) -> bool {
        self.is_space_available(space) && self.is_space_in_bounds(space)
    }

    fn is_space_available(&self, space: &i32) -> bool {
        !self.spaces.contains(space)
    }

    fn is_space_in_bounds(&self, space: &i32) -> bool {
        let max_space = &self.size * &self.size;
        let min_space = 0;
        space >= &min_space && space < &max_space
    }

    fn create_new_board_with_move(self, space: i32) -> Board {
        let mut updated_spaces = self.spaces;
        updated_spaces.push(space);
        Board {
            size: self.size,
            spaces: updated_spaces,
        }
    }

    pub fn expand_board(&self) -> Vec<String> {
        let spaces = &self.spaces;
        let number_of_spaces = self.size * self.size;
        let mut expanded_board: Vec<String> = vec![" ".to_string(); number_of_spaces as usize];
        for (index, space) in spaces.iter().enumerate() {
            if index % 2 == 0 {
                expanded_board[*space as usize] = "X".to_string();
            } else {
                expanded_board[*space as usize] = "O".to_string();
            }
        }
        expanded_board
    }
}

pub fn split_into_rows(expanded_board: Vec<String>, size: i32) -> Vec<Vec<String>> {
    let chunks = expanded_board.chunks(size as usize);
    let mut rows: Vec<Vec<String>> = Vec::new();
    for chunk in chunks {
        let row: Vec<String> = chunk.to_vec();
        rows.push(row);
    }
    rows
}

pub mod tests {
    use super::*;
    #[cfg(test)]
    #[test]
    fn takes_a_number_of_rows() {
        let board = build_board(3);
        assert_eq!(&3, board.get_size());
    }

    #[test]
    fn starts_with_no_moves() {
        let spaces: Vec<i32> = vec![];
        let board = set_up_board(3, vec![]);
        assert_eq!(&spaces, board.get_spaces());
    }

    #[test]
    fn x_plays_first() {
        let spaces = vec![0];
        let board = set_up_board(3, vec![0]);
        assert_eq!(&spaces, board.get_spaces());
    }

    #[test]
    fn o_plays_next() {
        let spaces = vec![0, 4];
        let board = set_up_board(3, vec![0, 4]);
        assert_eq!(&spaces, board.get_spaces());
    }

    #[test]
    fn a_space_can_only_be_taken_once() {
        let spaces = vec![0, 4];
        let board = set_up_board(3, vec![0, 4, 4]);
        assert_eq!(&spaces, board.get_spaces());
    }

    #[test]
    fn a_negative_space_cant_be_chosen() {
        let spaces = vec![0, 4];
        let board = set_up_board(3, vec![0, 4, -4]);
        assert_eq!(&spaces, board.get_spaces());
    }

    #[test]
    fn a_space_above_the_board_cant_be_chosen() {
        let spaces = vec![0, 4];
        let board = set_up_board(3, vec![0, 4, 9]);
        assert_eq!(&spaces, board.get_spaces());
    }

    #[test]
    fn convert_empty_board() {
        let board = set_up_board(3, vec![]);
        let expanded_board: Vec<String> = vec![" ".to_string(), " ".to_string(), " ".to_string(), " ".to_string(),
        " ".to_string(), " ".to_string(), " ".to_string(), " ".to_string(), " ".to_string()];
        assert_eq!(expanded_board, board.expand_board());
    }

    #[test]
    fn convert_in_progress_board() {
        let board = set_up_board(3, vec![0, 4]);
        let expanded_board: Vec<String> = vec!["X".to_string(), " ".to_string(), " ".to_string(), " ".to_string(),
        "O".to_string(), " ".to_string(), " ".to_string(), " ".to_string(), " ".to_string()];
        assert_eq!(expanded_board, board.expand_board());
    }

    #[test]
    fn convert_full_board() {
        let board = set_up_board(3, vec![0, 4, 8, 2, 6, 7, 1, 3, 5]);
        let expanded_board: Vec<String> = vec!["X".to_string(), "X".to_string(), "O".to_string(), "O".to_string(), "O".to_string(),
        "X".to_string(), "X".to_string(), "O".to_string(), "X".to_string()];
        assert_eq!(expanded_board, board.expand_board());
    }

    #[test]
    fn split_into_rows_empty() {
        let board = set_up_board(3, vec![]);
        let expanded_board: Vec<Vec<String>> = vec![vec![" ".to_string(), " ".to_string(), " ".to_string()],
        vec![" ".to_string(), " ".to_string(), " ".to_string()], vec![" ".to_string(), " ".to_string(), " ".to_string()]];
        assert_eq!(expanded_board, split_into_rows(board.expand_board(), 3));
    }

    #[test]
    fn split_into_rows_in_progress() {
        let board = set_up_board(3, vec![0, 4]);
        let expanded_board: Vec<Vec<String>> = vec![vec!["X".to_string(), " ".to_string(), " ".to_string()],
        vec![" ".to_string(), "O".to_string(), " ".to_string()], vec![" ".to_string(), " ".to_string(), " ".to_string()]];
        assert_eq!(expanded_board, split_into_rows(board.expand_board(), 3));
    }

    #[test]
    fn split_into_rows_full() {
        let board = set_up_board(3, vec![0, 4, 8, 2, 6, 7, 1, 3, 5]);
        let expanded_board: Vec<Vec<String>> = vec![vec!["X".to_string(), "X".to_string(), "O".to_string()],
        vec!["O".to_string(), "O".to_string(), "X".to_string()], vec!["X".to_string(), "O".to_string(), "X".to_string()]];
        assert_eq!(expanded_board, split_into_rows(board.expand_board(), 3));
    }

   pub fn set_up_board(size: i32, spaces: Vec<i32>) -> Board {
        let mut board: Board = build_board(size);
        for space in spaces {
            board = board.place_marker(space);
        }
        board
    }

}
