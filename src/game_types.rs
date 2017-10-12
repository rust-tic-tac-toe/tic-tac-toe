use players::*;

pub fn create_players(choice: i32) -> Vec<Players> {
        let x = Players::Human { marker: "X".to_string() };
        let o = Players::Human { marker: "O".to_string() };
        vec![x, o]
}

pub mod tests {
    use super::*;
    #[cfg(test)]
    #[test]
    fn creates_two_players() {
        let players = create_players(1);
        let x = &players[0];
        let o = &players[1];
        assert_eq!("X", get_marker(x));
        assert_eq!("O", get_marker(o));
    }
}

