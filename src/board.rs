pub fn build_board(size: i32) -> Board {
    Board {
        size,
        spaces: Vec::new(),
    }
}

#[derive(Clone, Default)]
pub struct Board {
    size: i32,
    spaces: Vec<i32>,
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
            self.create_next_board(space)
        } else {
            self
        }
    }

    fn is_move_valid(&self, space: &i32) -> bool {
        self.is_space_available(space) && self.is_space_in_bounds(space)
    }

    pub fn is_space_available(&self, space: &i32) -> bool {
        !self.spaces.contains(space)
    }

    fn is_space_in_bounds(&self, space: &i32) -> bool {
        let max_space = self.size * self.size;
        let min_space = 0;
        space >= &min_space && space < &max_space
    }

    pub fn get_available_spaces(&self) -> Vec<i32> {
        let all_spaces = 0..self.size * self.size;
        all_spaces
            .filter(|space| self.is_space_available(space))
            .collect()
    }

    fn create_next_board(self, space: i32) -> Board {
        let mut updated_spaces = self.spaces;
        updated_spaces.push(space);
        Board {
            size: self.size,
            spaces: updated_spaces,
        }
    }
}

pub mod tests {
    #[cfg(test)]
    use super::*;

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
    fn finds_available_spaces_in_empty_board() {
        let board = set_up_board(3, vec![]);
        let available_spaces = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
        assert_eq!(available_spaces, board.get_available_spaces());
    }

    #[test]
    fn finds_available_spaces_in_full_board() {
        let board = set_up_board(3, vec![0, 4, 8, 2, 6, 7, 1, 3, 5]);
        let available_spaces: Vec<i32> = vec![];
        assert_eq!(available_spaces, board.get_available_spaces());
    }

    #[test]
    fn finds_available_spaces_in_an_in_progress_board() {
        let board = set_up_board(3, vec![0, 4, 8, 2, 6]);
        let available_spaces: Vec<i32> = vec![1, 3, 5, 7];
        assert_eq!(available_spaces, board.get_available_spaces());
    }

    #[cfg(test)]
    pub fn set_up_board(size: i32, spaces: Vec<i32>) -> Board {
        let mut board: Board = build_board(size);
        for space in spaces {
            board = board.place_marker(space);
        }
        board
    }

}
