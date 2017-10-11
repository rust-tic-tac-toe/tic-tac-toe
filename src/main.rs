mod board;
mod app_runner;
mod io;

use app_runner::start;

fn main() {
    start();
}

#[cfg(test)]

mod tests {
    use super::*;
}
