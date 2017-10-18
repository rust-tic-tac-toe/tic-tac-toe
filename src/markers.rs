pub enum Markers {
    X,
    O,
}

pub fn inspect(marker: Markers) -> String {
    match marker {
        Markers::X => "X".to_string(),
        Markers::O => "O".to_string(),
    }
}

pub mod tests {
    #[cfg(test)]
    use super::*;

    #[test]
    fn has_x_as_a_marker() {
        let marker = Markers::X;
        assert_eq!("X", inspect(marker));
    }

    #[test]
    fn has_o_as_a_marker() {
        let marker = Markers::O;
        assert_eq!("O", inspect(marker));
    }
}
