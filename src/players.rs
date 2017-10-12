pub enum Players {
    Human { marker: String },
    Computer { marker: String },
}

pub fn get_marker(player: &Players) -> &str {
    match player {
        &Players::Human { ref marker }=> marker,
        &Players::Computer { ref marker } => marker,
    }
}

pub mod tests {
    use super::*;
    #[cfg(test)]
    #[test]
    fn creates_a_human_player() {
        let player = Players::Human { marker: "X".to_string() };
        assert_eq!("X", get_marker(&player));
    }
}

