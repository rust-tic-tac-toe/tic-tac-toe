use board::Board;
use board::OFFSET;
use board_formatter::expand_board;
use lines::split_board_into_rows;

pub fn format_board(board: &Board) -> String {
    let split_board = split_board_into_rows(&number_spaces(&expand_board(board)), board.get_size().abs());
    let mut formatted_board: String = "".to_string();
    for (index, row) in split_board.iter().enumerate() {
        let formatted_row = format_row(&row.to_vec());
        let length = formatted_row.len();
        formatted_board += &formatted_row;
        if index < row.len() - OFFSET {
            formatted_board += &"-".repeat(length - OFFSET);
            formatted_board += "\n";
        }
    }
    formatted_board
}

fn format_row(row: &[String]) -> String {
    let mut formatted_row: String = "".to_string();
    for (index, mark) in row.iter().enumerate() {
        formatted_row.push_str(" ");
        formatted_row.push_str(mark);
        if mark.len() == OFFSET {
            formatted_row.push_str(" ");
        }
        formatted_row.push_str(" ");
        if index < row.len() - OFFSET {
            formatted_row.push_str("|");
        } else {
            formatted_row.push_str("\n");
        }
    }
    formatted_row
}

fn number_spaces(spaces: &[String]) -> Vec<String> {
    let mut updated_spaces: Vec<String> = vec![" ".to_string(); spaces.len() as usize];
    for (index, space) in spaces.iter().enumerate() {
        if space == " " {
            let number = index + OFFSET;
            updated_spaces[index] = number.to_string();
        } else {
            updated_spaces[index] = space.to_string();
        }
    }
    updated_spaces
}

#[cfg(test)]
mod tests {
    use super::*;
    use board::tests::set_up_board;
    #[test]
    fn displays_an_empty_3_by_3_board() {
        let board: Board = set_up_board(3, vec![]);
        let blank_board: String =
            " 1  | 2  | 3  \n--------------\n 4  | 5  | 6  \n--------------\n 7  | 8  | 9  \n"
            .to_string();
        assert_eq!(blank_board, format_board(&board));
    }

    #[test]
    fn displays_a_full_3_by_3_board() {
        let board: Board = set_up_board(3, vec![0, 4, 8, 2, 6, 7, 1, 3, 5]);
        let blank_board: String =
            " X  | X  | O  \n--------------\n O  | O  | X  \n--------------\n X  | O  | X  \n"
            .to_string();
        assert_eq!(blank_board, format_board(&board));
    }

    #[test]
    fn formats_a_row() {
        let row: String = " 1  | 2  | 3  \n".to_string();
        assert_eq!(
            row,
            format_row(&vec!["1".to_string(), "2".to_string(), "3".to_string()])
            );
    }

    #[test]
    fn formats_numbers() {
        let numbered_spaces: Vec<String> = vec![
            "1".to_string(),
            "2".to_string(),
            "3".to_string(),
            "4".to_string(),
            "5".to_string(),
            "6".to_string(),
            "7".to_string(),
            "8".to_string(),
            "9".to_string(),
        ];
        assert_eq!(
            numbered_spaces,
            number_spaces(&vec![
                          " ".to_string(),
                          " ".to_string(),
                          " ".to_string(),
                          " ".to_string(),
                          " ".to_string(),
                          " ".to_string(),
                          " ".to_string(),
                          " ".to_string(),
                          " ".to_string(),
            ])
            );
    }

}
