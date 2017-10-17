extern crate termion;
use std::io::{self, BufRead};
use board::*;
use lines::split_board_into_rows;
use board_formatter::expand_board;

pub const TITLE: &str = "Tic Tac Toe";
pub const GAME_TYPE: &str = "Select game type
1 - Human vs Human
2 - Human vs Computer
3 - Computer vs Human
4 - Computer vs Computer";
pub const SELECT_A_SPACE: &str = ", select a space";
pub const WINNER: &str = " wins the game!";

pub fn display(output: &str) {
    println!("{}", output);
}

pub fn select_number() -> i32 {
    let input = get_input();
    match input.trim().parse::<i32>() {
        Ok(n) => n,
        Err(_e) => select_number(),
    }
}

pub fn ask_player_type() -> i32 {
    display(GAME_TYPE);
    let selection = select_number();
    if selection == 1 || selection == 2 || selection == 3 || selection == 4 {
        selection
    } else {
        ask_player_type()
    }
}

fn get_input() -> String {
    let stdio = io::stdin();
    let input = stdio.lock();
    process_input(input)
}

fn process_input<R>(mut reader: R) -> String
where
    R: BufRead,
{
    let mut input = String::new();
    reader.read_line(&mut input).expect("Unable to read");
    input
}

pub fn select_space(player: &str) -> String {
    let mut select: String = player.to_string();
    select += SELECT_A_SPACE;
    select
}

pub fn alert_winner(player: &str) -> String {
    let mut winner: String = player.to_string();
    winner += WINNER;
    winner
}

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

pub fn clear_screen() {
   print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
}

#[cfg(test)]
mod tests {
    use super::*;
    use board::tests::set_up_board;
    use std::io::Cursor;

    #[test]
    fn captures_user_input() {
        let input = Cursor::new(&b"3"[..]);
        let answer = process_input(input);
        assert_eq!("3", answer);
    }

    #[test]
    fn displays_the_title() {
        assert_eq!("Tic Tac Toe", TITLE);
    }

    #[test]
    fn asks_user_to_select_game_type() {
        assert_eq!(
            "Select game type
1 - Human vs Human
2 - Human vs Computer
3 - Computer vs Human
4 - Computer vs Computer",
            GAME_TYPE
        );
    }

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


    #[test]
    fn asks_user_to_play() {
        let turn: String = "X, select a space".to_string();
        assert_eq!(turn, select_space("X"));
    }

    #[test]
    fn alerts_user_of_the_winner() {
        let winner: String = "X wins the game!".to_string();
        assert_eq!(winner, alert_winner("X"));
    }

}
