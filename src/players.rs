use board::*;
use io::*;

pub enum Players {
    Human { marker: String },
    Computer { marker: String },
}

pub fn get_marker(player: &Players) -> &str {
    match *player {
        Players::Human { ref marker } | Players::Computer { ref marker } => marker,
    }
}

pub fn choose_space(player: &Players, board: &Board) -> i32 {
    match *player {
        Players::Human { ref marker } => ask(&select_space(marker)) - OFFSET as i32,
        Players::Computer { ref marker } => 1,
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
}
