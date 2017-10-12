pub enum Players {
    Human { marker: String },
    Computer { marker: String },
}

fn get_marker(player: Players) -> String {
    match player {
        Players::Human { marker }=> marker,
        Players::Computer { marker } => marker,
    }
}

pub mod tests {
    use super::*;
    #[cfg(test)]
    #[test]
    fn creates_a_human_player() {
        let player = Players::Human { marker: "X".to_string() };
        assert_eq!("X", get_marker(player));
    }
}

