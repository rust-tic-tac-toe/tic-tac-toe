extern crate termion;
use marker;
use marker::Marker;
use std::io::{self, BufRead};

pub const TITLE: &str = "Tic Tac Toe";
pub const GAME_TYPE: &str = "Select game type
1 - Human vs Human
2 - Human vs Computer
3 - Computer vs Human
4 - Computer vs Computer";
pub const SELECT_A_SPACE: &str = ", select a space";
pub const WINNER: &str = " wins the game!";
pub const PLAY_AGAIN: &str = "Play again?
1 - Yes
2 - No";

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

pub fn ask_play_again() -> i32 {
    display(PLAY_AGAIN);
    let selection = select_number();
    if selection == 1 || selection == 2 {
        selection
    } else {
        ask_play_again()
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

pub fn select_space(player: &Marker) -> String {
    let mut select: String = marker::inspect(player);
    select += SELECT_A_SPACE;
    select
}

pub fn alert_winner(player: &Marker) -> String {
    let mut winner: String = marker::inspect(player);
    winner += WINNER;
    winner
}

pub fn clear_screen() {
    print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
}

#[cfg(test)]
mod tests {
    use super::*;
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
    fn asks_user_to_play_again() {
        assert_eq!(
            "Play again?
1 - Yes
2 - No",
            PLAY_AGAIN
        );
    }

    #[test]
    fn asks_user_to_play() {
        let turn: String = "X, select a space".to_string();
        assert_eq!(turn, select_space(&Marker::X));
    }

    #[test]
    fn alerts_user_of_the_winner() {
        let winner: String = "X wins the game!".to_string();
        assert_eq!(winner, alert_winner(&Marker::X));
    }

}
