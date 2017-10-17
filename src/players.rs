use computer;
use human;
use board::Board;

pub enum Players {
    Human { marker: String },
    Computer { marker: String },
}

pub fn get_marker(player: &Players) -> &str {
    match *player {
        Players::Human { ref marker } | Players::Computer { ref marker } => marker,
    }
}

#[allow(unused)]
pub fn choose_space(player: &Players, board: &Board) -> i32 {
    match *player {
        Players::Human { ref marker } => human::ask_user_to_select_space(),
        Players::Computer { ref marker } => computer::find_best_space(board),
    }
}

pub mod tests {
    #[cfg(test)]
    use super::*;
    #[test]
    fn creates_a_human_player() {
        let player = Players::Human {
            marker: "X".to_string(),
        };
        assert_eq!("X", get_marker(&player));
    }

    #[test]
    fn creates_a_computer_player() {
        let player = Players::Computer {
            marker: "O".to_string(),
        };
        assert_eq!("O", get_marker(&player));
    }
}
