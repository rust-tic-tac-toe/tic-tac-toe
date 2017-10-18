use computer;
use human;
use marker::Marker;
use board::Board;

pub enum Players {
    Human { marker: Marker },
    Computer { marker: Marker },
}

pub fn get_marker(player: &Players) -> &Marker {
    match *player {
        Players::Human { ref marker } | Players::Computer { ref marker } => marker,
    }
}

#[allow(unused)]
pub fn choose_space(player: &Players, board: &Board) -> i32 {
    match *player {
        Players::Human { ref marker } => human::find_space(),
        Players::Computer { ref marker } => computer::find_space(board),
    }
}

pub mod tests {
    #[cfg(test)]
    use super::*;

    #[test]
    fn creates_a_human_player() {
        let player = Players::Human { marker: Marker::X };
        assert_eq!(&Marker::X, get_marker(&player));
    }

    #[test]
    fn creates_a_computer_player() {
        let player = Players::Computer { marker: Marker::O };
        assert_eq!(&Marker::O, get_marker(&player));
    }
}
