struct Board {
    size: i32,
    spaces: Vec<i32>,
}

fn build_board(size: i32) -> Board {
    Board {
        size,
        spaces: Vec::new(),
    }
}

impl Board {

    fn place_marker(self, space: i32) -> Board {
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

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn takes_a_number_of_rows() {
        let board = build_board(3);
        assert_eq!(3, board.size);
    }

    #[test]
    fn starts_with_no_moves() {
        let spaces: Vec<i32> = vec![];
        let board = set_up_board(3, vec![]);
        assert_eq!(spaces, board.spaces);
    }


    #[test]
    fn x_plays_first() {
        let spaces: Vec<i32> = vec![0];
        let board = set_up_board(3, vec![0]);
        assert_eq!(spaces, board.spaces);
    }

    #[test]
    fn o_plays_next() {
        let spaces: Vec<i32> = vec![0, 4];
        let board = set_up_board(3, vec![0, 4]);
        assert_eq!(spaces, board.spaces);
    }

    #[test]
    fn a_space_can_only_be_taken_once() {
        let spaces: Vec<i32> = vec![0, 4];
        let board = set_up_board(3, vec![0, 4, 4]);
        assert_eq!(spaces, board.spaces);
    }

    #[test]
    fn a_negative_space_cant_be_chosen() {
        let spaces: Vec<i32> = vec![0, 4];
        let board = set_up_board(3, vec![0, 4, -4]);
        assert_eq!(spaces, board.spaces);
    }

    #[test]
    fn a_space_above_the_board_cant_be_chosen() {
        let spaces: Vec<i32> = vec![0, 4];
        let board = set_up_board(3, vec![0, 4, 9]);
        assert_eq!(spaces, board.spaces);
    }

    fn set_up_board(size: i32, spaces: Vec<i32>) -> Board {
        let mut board: Board = build_board(size);
        for space in spaces {
            board = board.place_marker(space);
        }
        board
    }

}
