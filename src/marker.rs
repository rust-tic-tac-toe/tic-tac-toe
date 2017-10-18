#[derive(PartialEq, Debug)]
pub enum Marker {
    X,
    O,
}

pub fn inspect(marker: &Marker) -> String {
    match marker {
        &Marker::X => "X".to_string(),
        &Marker::O => "O".to_string(),
    }
}

pub mod tests {
    #[cfg(test)]
    use super::*;

    #[test]
    fn has_x_as_a_marker() {
        let marker = Marker::X;
        assert_eq!("X", inspect(&marker));
    }

    #[test]
    fn has_o_as_a_marker() {
        let marker = Marker::O;
        assert_eq!("O", inspect(&marker));
    }
}
