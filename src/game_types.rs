use players::Players;
use marker::Marker;

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
        marker: Marker::X,
    };
    let o = Players::Human {
        marker: Marker::O,
    };
    vec![x, o]
}

fn human_vs_computer() -> Vec<Players> {
    let x = Players::Human {
        marker: Marker::X,
    };
    let o = Players::Computer {
        marker: Marker::O,
    };
    vec![x, o]
}


fn computer_vs_human() -> Vec<Players> {
    let x = Players::Computer {
        marker: Marker::X,
    };
    let o = Players::Human {
        marker: Marker::O,
    };
    vec![x, o]
}

fn computer_vs_computer() -> Vec<Players> {
    let x = Players::Computer {
        marker: Marker::X,
    };
    let o = Players::Computer {
        marker: Marker::O,
    };
    vec![x, o]
}

pub mod tests {
    #[cfg(test)]
    use super::*;
    #[cfg(test)]
    use players;
    #[test]
    fn creates_two_players() {
        let players = create_players(1);
        let x = &players[0];
        let o = &players[1];
        assert_eq!(&Marker::X, players::get_marker(x));
        assert_eq!(&Marker::O, players::get_marker(o));
    }
}
