[![Build Status](https://travis-ci.org/rust-tic-tac-toe/tic-tac-toe.svg?branch=master)](https://travis-ci.org/rust-tic-tac-toe/tic-tac-toe)
# Rust Tic Tac Toe

This is a simple Tic Tac Toe game built in Rust. There are four options to play
the game: 

1. Human vs Human
2. Human vs Computer
3. Computer vs Human
4. Computer vs Computer

The computer player uses the [minimax algorithm](https://en.wikipedia.org/wiki/Minimax)
to select the best space.

#### Prerequisites
1. Install Rust by following [these instructions](https://www.rust-lang.org/en-US/install.htm), followed by the instructions in your terminal. 

#### Running instructions
1. In your home folder, clone the repository `$ git clone
git@github.com:rust-tic-tac-toe/tic-tac-toe.git`
2. In terminal, CD into the repository `$cd tic-tac-toe`
3. Build the game by running `$ cargo build`
4. Test the game by running `$ cargo test`
5. Play the game by running `$ cargo run`

#### Note
- Per the [Test
  Organization](https://doc.rust-lang.org/book/second-edition/ch11-03-test-organization.html)
  chapter of [The Rust Programming
  Language](https://doc.rust-lang.org/book/second-edition/); unit tests are
  included in the source file they are testing and integration tests are
  separate in the tests folder. 
- There are no unit tests for `app_runner.rs` and `human.rs` as this code is tested by the integration tests. The functions called in these two files are tested in their relative source files.
