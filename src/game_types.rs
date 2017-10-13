use players::*;

pub fn create_players(choice: i32) -> Vec<Players> {
    match choice {
        1 => human_vs_human(),
        2 => human_vs_computer(),
        3 => computer_vs_human(),
        _ => computer_vs_computer(),
    }
}

fn human_vs_human() -> Vec<Players> {
    let x = Players::Human {
        marker: "X".to_string(),
    };
    let o = Players::Human {
        marker: "O".to_string(),
    };
    vec![x, o]
}

fn human_vs_computer() -> Vec<Players> {
    let x = Players::Human {
        marker: "X".to_string(),
    };
    let o = Players::Computer {
        marker: "O".to_string(),
    };
    vec![x, o]
}


fn computer_vs_human() -> Vec<Players> {
    let x = Players::Computer {
        marker: "X".to_string(),
    };
    let o = Players::Human {
        marker: "O".to_string(),
    };
    vec![x, o]
}

fn computer_vs_computer() -> Vec<Players> {
    let x = Players::Computer {
        marker: "X".to_string(),
    };
    let o = Players::Computer {
        marker: "O".to_string(),
    };
    vec![x, o]
}

pub mod tests {
    #[cfg(test)]
    use super::*;
    #[test]
    fn creates_two_players() {
        let players = create_players(1);
        let x = &players[0];
        let o = &players[1];
        assert_eq!("X", get_marker(x));
        assert_eq!("O", get_marker(o));
    }
}
