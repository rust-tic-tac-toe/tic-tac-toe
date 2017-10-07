#[cfg(not(test))]
fn main() {
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!("a", "a");
    }
}
