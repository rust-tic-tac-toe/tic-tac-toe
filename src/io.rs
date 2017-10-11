use std::io::{self, BufRead, Write, Cursor};
use board::Board;
use board::split_into_rows;
use board::tests::set_up_board;

pub const OFFSET: usize = 1;
pub const TITLE: &str = "Tic Tac Toe";
const INPUT_MARKER: &str = "::: ";
pub const NUMBER_OF_ROWS: &str = "Select 3 or 4 rows to play";
pub const SELECT_A_SPACE: &str = ", select a space";
pub const WINNER: &str = " wins the game!";

pub fn display(output: &str) {
    println!("{}", output);
}

pub fn ask(question: &str) -> i32 {
    display(question);
    get_input().trim().parse::<i32>().unwrap()
}

fn get_input() -> String {
    let stdio = io::stdin();
    let input = stdio.lock();
    let output = io::stdout();
    let answer = process_input(input, output, INPUT_MARKER);
    display(&answer);
    answer
}

fn process_input<R, W>(mut reader: R, mut writer: W, query: &str) -> String
where R: BufRead, W: Write {
    write!(&mut writer, "{}", query).expect("Unable to write");
    let mut input = String::new();
    reader.read_line(&mut input).expect("Unable to read");
    input
}

pub fn select_space(player: String) -> String {
    let mut select: String = player.to_string();
    select += SELECT_A_SPACE;
    select
}

pub fn alert_winner(player: String) -> String {
    let mut winner: String = player.to_string();
    winner += WINNER;
    winner
}

pub fn format_board(board: &Board) -> String {
    let split_board = split_into_rows(number_spaces(board.expand_board()), board.get_size().abs());
    let mut formatted_board: String = "".to_string();
    for (index, row) in split_board.iter().enumerate() {
        let formatted_row = format_row(row.to_vec());
        let length = formatted_row.len();
        formatted_board += &formatted_row;
        if index < row.len() - OFFSET {
            formatted_board += &"-".repeat(length - OFFSET);
            formatted_board += "\n";
        }
    }
    formatted_board
}

fn format_row(row: Vec<String>) -> String {
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

fn number_spaces(spaces: Vec<String>) -> Vec<String> {
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

pub fn clear_screen() {
    print!("{}[2J", 27 as char);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn displays_input_marker() {
        let input = Cursor::new(&b"3"[..]);
        let mut output = Cursor::new(Vec::new());
        process_input(input, &mut output, INPUT_MARKER);
        let output = String::from_utf8(output.into_inner()).expect("Not UTF-8");
        assert_eq!(INPUT_MARKER, output);
    }

    #[test]
    fn captures_user_input() {
        let input = Cursor::new(&b"3"[..]);
        let mut output = Cursor::new(Vec::new());
        let answer = process_input(input, &mut output, NUMBER_OF_ROWS);
        assert_eq!("3", answer);
    }

    #[test]
    fn displays_the_title() {
        assert_eq!("Tic Tac Toe", TITLE);
    }

#[test]
    fn displays_an_empty_3_by_3_board() {
        let board: Board = set_up_board(3, vec![]);
        let blank_board: String =
            " 1  | 2  | 3  \n--------------\n 4  | 5  | 6  \n--------------\n 7  | 8  | 9  \n".to_string();
        assert_eq!(blank_board, format_board(&board));
    }

#[test]
    fn displays_a_full_3_by_3_board() {
        let board: Board = set_up_board(3, vec![0, 4, 8, 2, 6, 7, 1, 3, 5]);
        let blank_board: String =
            " X  | X  | O  \n--------------\n O  | O  | X  \n--------------\n X  | O  | X  \n".to_string();
        assert_eq!(blank_board, format_board(&board));
    }

#[test]
    fn displays_an_empty_4_by_4_board() {
        let board: Board = set_up_board(4, vec![]);
        let blank_board: String =
            " 1  | 2  | 3  | 4  \n-------------------\n 5  | 6  | 7  | 8  \n-------------------\n \
           9  | 10 | 11 | 12 \n-------------------\n 13 | 14 | 15 | 16 \n".to_string();
        assert_eq!(blank_board, format_board(&board));
    }

#[test]
    fn displays_a_full_4_by_4_board() {
        let board: Board = set_up_board(4, vec![0, 15, 1, 14, 2, 3, 7, 13, 12, 4, 5, 6, 8, 9, 10, 11]);
        let blank_board: String =
            " X  | X  | X  | O  \n-------------------\n O  | X  | O  | X  \n-------------------\n \
           X  | O  | X  | O  \n-------------------\n X  | O  | O  | O  \n".to_string();
        assert_eq!(blank_board, format_board(&board));
    }

#[test]
    fn formats_a_row() {
        let row: String =
            " 1  | 2  | 3  \n".to_string();
        assert_eq!(row, format_row(vec!["1".to_string(), "2".to_string(), "3".to_string()]));
    }

#[test]
    fn formats_numbers() {
        let numbered_spaces: Vec<String> =
            vec!["1".to_string(), "2".to_string(), "3".to_string(), "4".to_string(),
            "5".to_string(), "6".to_string(), "7".to_string(), "8".to_string(), "9".to_string()];
        assert_eq!(numbered_spaces, number_spaces(vec![" ".to_string(), " ".to_string(), " ".to_string(),
        " ".to_string(), " ".to_string(), " ".to_string(), " ".to_string(), " ".to_string(), " ".to_string()]));
    }


#[test]
    fn asks_user_to_play() {
        let turn: String = "X, select a space".to_string();
        assert_eq!(turn, select_space("X".to_string()));
    }

#[test]
    fn alerts_user_of_the_winner() {
        let winner: String = "X wins the game!".to_string();
        assert_eq!(winner, alert_winner("X".to_string()));
    }

}
