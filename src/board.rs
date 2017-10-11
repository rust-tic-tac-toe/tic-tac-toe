const OFFSET: usize = 1;

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

    fn get_available_spaces(&self) -> Vec<i32> {
        let all_spaces = 0..self.size * self.size;
        all_spaces.filter(|space| self.is_space_available(space)).collect()
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

fn is_game_tied(board: &Board) -> bool {
    !is_game_won(&board) && board.get_available_spaces().len() == 0
}

fn is_game_won(board: &Board) -> bool {
    is_game_won_by(&board, "X") || is_game_won_by(&board, "O")
}

fn is_game_won_by(board: &Board, player: &str) -> bool {
    let winning_scenarios = find_winning_scenarios(&board);
    winning_scenarios.iter().any(|line| is_line_won_by(line, &player))
}

fn is_line_won_by(line: &Vec<String>, player: &str) -> bool {
    line.iter().all(|space| space == player)
}

fn find_winning_scenarios(board: &Board) -> Vec<Vec<String>> {
    let mut winning_scenarios: Vec<Vec<String>> = Vec::new();
    let mut rows = split_into_rows(board.expand_board(), board.size);
    let mut columns = find_columns(&rows);
    let left = find_left_diagonal(&rows);
    let right = find_right_diagonal(&rows);
    winning_scenarios.append(&mut rows);
    winning_scenarios.append(&mut columns);
    winning_scenarios.push(left);
    winning_scenarios.push(right);
    winning_scenarios
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

fn find_columns(rows: &Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut columns = rows.to_vec();
    for (row_index, row) in rows.iter().enumerate() {
        for (space_index, space) in row.iter().enumerate() {
            columns[space_index][row_index] = space.to_string();
        }
    }
    columns
}

fn find_left_diagonal(rows: &Vec<Vec<String>>) -> Vec<String> {
    let mut diagonal: Vec<String> = vec![" ".to_string(); rows.len()];
    for (index, row) in rows.iter().enumerate() {
        diagonal[index] = row[index].to_string();
    }
    diagonal
}

fn find_right_diagonal(rows: &Vec<Vec<String>>) -> Vec<String> {
    let mut diagonal: Vec<String> = vec![" ".to_string(); rows.len()];
    for (index, row) in rows.iter().enumerate() {
        diagonal[index] = row[rows.len() - (index + OFFSET)].to_string();
    }
    diagonal
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

    #[test]
    fn get_columns_empty() {
        let rows: Vec<Vec<String>> = vec![vec![" ".to_string(), " ".to_string(), " ".to_string()],
        vec![" ".to_string(), " ".to_string(), " ".to_string()], vec![" ".to_string(), " ".to_string(), " ".to_string()]];
        assert_eq!(rows, find_columns(&rows));
    }

    #[test]
    fn get_columns_in_progress() {
        let rows: Vec<Vec<String>> = vec![vec!["X".to_string(), " ".to_string(), "X".to_string()],
        vec![" ".to_string(), "O".to_string(), " ".to_string()], vec![" ".to_string(), " ".to_string(), " ".to_string()]];
        let columns: Vec<Vec<String>> = vec![vec!["X".to_string(), " ".to_string(), " ".to_string()],
        vec![" ".to_string(), "O".to_string(), " ".to_string()], vec!["X".to_string(), " ".to_string(), " ".to_string()]];
        assert_eq!(columns, find_columns(&rows));
    }

    #[test]
    fn get_columns_full() {
        let rows: Vec<Vec<String>> = vec![vec!["X".to_string(), "X".to_string(), "O".to_string()],
        vec!["O".to_string(), "O".to_string(), "X".to_string()], vec!["X".to_string(), "O".to_string(), "X".to_string()]];
        let columns: Vec<Vec<String>> = vec![vec!["X".to_string(), "O".to_string(), "X".to_string()],
        vec!["X".to_string(), "O".to_string(), "O".to_string()], vec!["O".to_string(), "X".to_string(), "X".to_string()]];
        assert_eq!(columns, find_columns(&rows));
    }

    #[test]
    fn get_columns_4x4() {
        let rows: Vec<Vec<String>> = vec![vec!["X".to_string(), " ".to_string(), "O".to_string(), " ".to_string()],
        vec![" ".to_string(), " ".to_string(), " ".to_string(), "X".to_string()],
        vec![" ".to_string(), "X".to_string(), " ".to_string(), " ".to_string()],
        vec!["O".to_string(), " ".to_string(), " ".to_string(), "O".to_string()]];
        let columns: Vec<Vec<String>> = vec![vec!["X".to_string(), " ".to_string(), " ".to_string(), "O".to_string()],
        vec![" ".to_string(), " ".to_string(), "X".to_string(), " ".to_string()],
        vec!["O".to_string(), " ".to_string(), " ".to_string(), " ".to_string()],
        vec![" ".to_string(), "X".to_string(), " ".to_string(), "O".to_string()]];
        assert_eq!(columns, find_columns(&rows));
    }

    #[test]
    fn get_left_diagonal_3x3() {
        let rows: Vec<Vec<String>> = vec![vec!["X".to_string(), "X".to_string(), "O".to_string()],
        vec!["O".to_string(), "O".to_string(), "X".to_string()], vec!["X".to_string(), "O".to_string(), "X".to_string()]];
        let diagonal: Vec<String> = vec!["X".to_string(), "O".to_string(), "X".to_string()];
        assert_eq!(diagonal, find_left_diagonal(&rows));
    }

    #[test]
    fn get_left_diagonal_4x4() {
        let rows: Vec<Vec<String>> = vec![vec!["X".to_string(), " ".to_string(), "O".to_string(), " ".to_string()],
        vec![" ".to_string(), " ".to_string(), " ".to_string(), "X".to_string()],
        vec![" ".to_string(), "X".to_string(), " ".to_string(), " ".to_string()],
        vec!["O".to_string(), " ".to_string(), " ".to_string(), "O".to_string()]];
        let diagonal: Vec<String> = vec!["X".to_string(), " ".to_string(), " ".to_string(), "O".to_string()];
        assert_eq!(diagonal, find_left_diagonal(&rows));
    }

    #[test]
    fn get_right_diagonal_3x3() {
        let rows: Vec<Vec<String>> = vec![vec!["X".to_string(), "X".to_string(), "O".to_string()],
        vec!["O".to_string(), "O".to_string(), "X".to_string()], vec!["X".to_string(), "O".to_string(), "X".to_string()]];
        let diagonal: Vec<String> = vec!["O".to_string(), "O".to_string(), "X".to_string()];
        assert_eq!(diagonal, find_right_diagonal(&rows));
    }

    #[test]
    fn get_right_diagonal_4x4() {
        let rows: Vec<Vec<String>> = vec![vec!["X".to_string(), " ".to_string(), "O".to_string(), " ".to_string()],
        vec![" ".to_string(), " ".to_string(), " ".to_string(), "X".to_string()],
        vec![" ".to_string(), "X".to_string(), " ".to_string(), " ".to_string()],
        vec!["O".to_string(), " ".to_string(), " ".to_string(), "O".to_string()]];
        let diagonal: Vec<String> = vec![" ".to_string(), " ".to_string(), "X".to_string(), "O".to_string()];
        assert_eq!(diagonal, find_right_diagonal(&rows));
    }

    #[test]
    fn finds_winning_scenarios() {
        let board = set_up_board(3, vec![0, 4, 8, 2, 6, 7, 1, 3, 5]);
        let winning_scenarios: Vec<Vec<String>> = vec![vec!["X".to_string(), "X".to_string(), "O".to_string()],
        vec!["O".to_string(), "O".to_string(), "X".to_string()], vec!["X".to_string(), "O".to_string(), "X".to_string()],
        vec!["X".to_string(), "O".to_string(), "X".to_string()], vec!["X".to_string(), "O".to_string(), "O".to_string()],
        vec!["O".to_string(), "X".to_string(), "X".to_string()], vec!["X".to_string(), "O".to_string(), "X".to_string()],
        vec!["O".to_string(), "O".to_string(), "X".to_string()]];
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

    pub fn set_up_board(size: i32, spaces: Vec<i32>) -> Board {
        let mut board: Board = build_board(size);
        for space in spaces {
            board = board.place_marker(space);
        }
        board
    }

}
