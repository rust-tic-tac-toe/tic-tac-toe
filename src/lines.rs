use board::*;
use board_formatter::expand_board;

pub fn split_board_into_rows(expanded_board: &[String], size: i32) -> Vec<Vec<String>> {
    let chunks = expanded_board.chunks(size as usize);
    let mut rows: Vec<Vec<String>> = Vec::new();
    for chunk in chunks {
        let row: Vec<String> = chunk.to_vec();
        rows.push(row);
    }
    rows
}

pub fn find_columns(rows: &[Vec<String>]) -> Vec<Vec<String>> {
    let mut columns = rows.to_vec();
    for (row_index, row) in rows.iter().enumerate() {
        for (space_index, space) in row.iter().enumerate() {
            columns[space_index][row_index] = space.to_string();
        }
    }
    columns
}

pub fn find_left_diagonal(rows: &[Vec<String>]) -> Vec<String> {
    let mut diagonal: Vec<String> = vec![" ".to_string(); rows.len()];
    for (index, row) in rows.iter().enumerate() {
        diagonal[index] = row[index].to_string();
    }
    diagonal
}

pub fn find_right_diagonal(rows: &[Vec<String>]) -> Vec<String> {
    let mut diagonal: Vec<String> = vec![" ".to_string(); rows.len()];
    for (index, row) in rows.iter().enumerate() {
        diagonal[index] = row[rows.len() - (index + OFFSET)].to_string();
    }
    diagonal
}

pub fn find_all_lines(board: &Board) -> Vec<Vec<String>> {
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

pub mod tests {
    #[cfg(test)]
    use super::*;
    #[cfg(test)]
    use board::tests::set_up_board;

    #[test]
    fn split_into_rows_empty() {
        let board = set_up_board(3, vec![]);
        let expanded_board: Vec<Vec<String>> = vec![
            vec![" ".to_string(), " ".to_string(), " ".to_string()],
            vec![" ".to_string(), " ".to_string(), " ".to_string()],
            vec![" ".to_string(), " ".to_string(), " ".to_string()],
        ];
            assert_eq!(expanded_board, split_board_into_rows(&expand_board(&board), 3));
    }

    #[test]
    fn split_into_rows_in_progress() {
        let board = set_up_board(3, vec![0, 4]);
        let expanded_board: Vec<Vec<String>> = vec![
            vec!["X".to_string(), " ".to_string(), " ".to_string()],
            vec![" ".to_string(), "O".to_string(), " ".to_string()],
            vec![" ".to_string(), " ".to_string(), " ".to_string()],
        ];
            assert_eq!(expanded_board, split_board_into_rows(&expand_board(&board), 3));
    }

    #[test]
    fn split_into_rows_full() {
        let board = set_up_board(3, vec![0, 4, 8, 2, 6, 7, 1, 3, 5]);
        let expanded_board: Vec<Vec<String>> = vec![
            vec!["X".to_string(), "X".to_string(), "O".to_string()],
            vec!["O".to_string(), "O".to_string(), "X".to_string()],
            vec!["X".to_string(), "O".to_string(), "X".to_string()],
        ];
            assert_eq!(expanded_board, split_board_into_rows(&expand_board(&board), 3));
    }

    #[test]
    fn get_columns_empty() {
        let rows: Vec<Vec<String>> = vec![
            vec![" ".to_string(), " ".to_string(), " ".to_string()],
            vec![" ".to_string(), " ".to_string(), " ".to_string()],
            vec![" ".to_string(), " ".to_string(), " ".to_string()],
        ];
            assert_eq!(rows, find_columns(&rows));
    }

    #[test]
    fn get_columns_in_progress() {
        let rows: Vec<Vec<String>> = vec![
            vec!["X".to_string(), " ".to_string(), "X".to_string()],
            vec![" ".to_string(), "O".to_string(), " ".to_string()],
            vec![" ".to_string(), " ".to_string(), " ".to_string()],
        ];
            let columns: Vec<Vec<String>> = vec![
                vec!["X".to_string(), " ".to_string(), " ".to_string()],
                vec![" ".to_string(), "O".to_string(), " ".to_string()],
                vec!["X".to_string(), " ".to_string(), " ".to_string()],
            ];
                assert_eq!(columns, find_columns(&rows));
    }

    #[test]
    fn get_columns_full() {
        let rows: Vec<Vec<String>> = vec![
            vec!["X".to_string(), "X".to_string(), "O".to_string()],
            vec!["O".to_string(), "O".to_string(), "X".to_string()],
            vec!["X".to_string(), "O".to_string(), "X".to_string()],
        ];
            let columns: Vec<Vec<String>> = vec![
                vec!["X".to_string(), "O".to_string(), "X".to_string()],
                vec!["X".to_string(), "O".to_string(), "O".to_string()],
                vec!["O".to_string(), "X".to_string(), "X".to_string()],
            ];
                assert_eq!(columns, find_columns(&rows));
    }

    #[test]
    fn get_left_diagonal_3x3() {
        let rows: Vec<Vec<String>> = vec![
            vec!["X".to_string(), "X".to_string(), "O".to_string()],
            vec!["O".to_string(), "O".to_string(), "X".to_string()],
            vec!["X".to_string(), "O".to_string(), "X".to_string()],
        ];
            let diagonal: Vec<String> = vec!["X".to_string(), "O".to_string(), "X".to_string()];
            assert_eq!(diagonal, find_left_diagonal(&rows));
    }

    #[test]
    fn get_right_diagonal_3x3() {
        let rows: Vec<Vec<String>> = vec![
            vec!["X".to_string(), "X".to_string(), "O".to_string()],
            vec!["O".to_string(), "O".to_string(), "X".to_string()],
            vec!["X".to_string(), "O".to_string(), "X".to_string()],
        ];
            let diagonal: Vec<String> = vec!["O".to_string(), "O".to_string(), "X".to_string()];
            assert_eq!(diagonal, find_right_diagonal(&rows));
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
            assert_eq!(winning_scenarios, find_all_lines(&board));
    }

}
