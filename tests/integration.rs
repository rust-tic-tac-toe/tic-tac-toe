extern crate assert_cli;

#[cfg(test)]
mod integration {
    use assert_cli;

    #[test]
    fn human_vs_human_x_wins() {
        assert_cli::Assert::main_binary()
            .stdin("1\n1\n5\n2\n4\n3\n2")
            .stdout()
            .contains("Tic Tac Toe")
            .stdout()
            .contains("Select game type")
            .stdout()
            .contains(
                "1 - Human vs Human\n2 - Human vs Computer\n\
                 3 - Computer vs Human\n4 - Computer vs Computer",
            )
            .stdout()
            .contains("X, select a space")
            .stdout()
            .contains(
                " 1  | 2  | 3  \n--------------\n 4  | 5  | 6  \n\
                 --------------\n 7  | 8  | 9  \n",
            )
            .stdout()
            .contains(
                " X  | 2  | 3  \n--------------\n 4  | 5  | 6  \n\
                 --------------\n 7  | 8  | 9  \n",
            )
            .stdout()
            .contains(
                " X  | 2  | 3  \n--------------\n 4  | O  | 6  \n\
                 --------------\n 7  | 8  | 9  \n",
            )
            .stdout()
            .contains(
                " X  | X  | 3  \n--------------\n 4  | O  | 6  \n\
                 --------------\n 7  | 8  | 9  \n",
            )
            .stdout()
            .contains(
                " X  | X  | 3  \n--------------\n O  | O  | 6  \n\
                 --------------\n 7  | 8  | 9  \n",
            )
            .stdout()
            .contains(
                " X  | X  | X  \n--------------\n O  | O  | 6  \n\
                 --------------\n 7  | 8  | 9  \n",
            )
            .stdout()
            .contains("X wins the game")
            .unwrap();
    }

    #[test]
    fn human_vs_human_o_wins() {
        assert_cli::Assert::main_binary()
            .stdin("1\n1\n5\n2\n4\n7\n6\n2")
            .stdout()
            .contains("Tic Tac Toe")
            .stdout()
            .contains("Select game type")
            .stdout()
            .contains(
                "1 - Human vs Human\n2 - Human vs Computer\n\
                 3 - Computer vs Human\n4 - Computer vs Computer",
            )
            .stdout()
            .contains("X, select a space")
            .stdout()
            .contains(
                " 1  | 2  | 3  \n--------------\n 4  | 5  | 6  \n\
                 --------------\n 7  | 8  | 9  \n",
            )
            .stdout()
            .contains(
                " X  | 2  | 3  \n--------------\n 4  | 5  | 6  \n\
                 --------------\n 7  | 8  | 9  \n",
            )
            .stdout()
            .contains(
                " X  | 2  | 3  \n--------------\n 4  | O  | 6  \n\
                 --------------\n 7  | 8  | 9  \n",
            )
            .stdout()
            .contains(
                " X  | X  | 3  \n--------------\n 4  | O  | 6  \n\
                 --------------\n 7  | 8  | 9  \n",
            )
            .stdout()
            .contains(
                " X  | X  | 3  \n--------------\n O  | O  | 6  \n\
                 --------------\n 7  | 8  | 9  \n",
            )
            .stdout()
            .contains(
                " X  | X  | 3  \n--------------\n O  | O  | 6  \n\
                 --------------\n X  | 8  | 9  \n",
            )
            .stdout()
            .contains(
                " X  | X  | 3  \n--------------\n O  | O  | O  \n\
                 --------------\n X  | 8  | 9  \n",
            )
            .stdout()
            .contains("O wins the game")
            .unwrap();
    }

    #[test]
    fn human_vs_human_nobody_wins() {
        assert_cli::Assert::main_binary()
            .stdin("1\n1\n5\n2\n4\n7\n8\n6\n3\n9\n2")
            .stdout()
            .contains("Tic Tac Toe")
            .stdout()
            .contains("Select game type")
            .stdout()
            .contains(
                "1 - Human vs Human\n2 - Human vs Computer\n\
                 3 - Computer vs Human\n4 - Computer vs Computer",
            )
            .stdout()
            .contains("X, select a space")
            .stdout()
            .contains(
                " 1  | 2  | 3  \n--------------\n 4  | 5  | 6  \n\
                 --------------\n 7  | 8  | 9  \n",
            )
            .stdout()
            .contains(
                " X  | 2  | 3  \n--------------\n 4  | 5  | 6  \n\
                 --------------\n 7  | 8  | 9  \n",
            )
            .stdout()
            .contains(
                " X  | 2  | 3  \n--------------\n 4  | O  | 6  \n\
                 --------------\n 7  | 8  | 9  \n",
            )
            .stdout()
            .contains(
                " X  | X  | 3  \n--------------\n 4  | O  | 6  \n\
                 --------------\n 7  | 8  | 9  \n",
            )
            .stdout()
            .contains(
                " X  | X  | 3  \n--------------\n O  | O  | 6  \n\
                 --------------\n 7  | 8  | 9  \n",
            )
            .stdout()
            .contains(
                " X  | X  | 3  \n--------------\n O  | O  | 6  \n\
                 --------------\n X  | 8  | 9  \n",
            )
            .stdout()
            .contains(
                " X  | X  | 3  \n--------------\n O  | O  | 6  \n\
                 --------------\n X  | O  | 9  \n",
            )
            .stdout()
            .contains(
                " X  | X  | 3  \n--------------\n O  | O  | X  \n\
                 --------------\n X  | O  | 9  \n",
            )
            .stdout()
            .contains(
                " X  | X  | O  \n--------------\n O  | O  | X  \n\
                 --------------\n X  | O  | 9  \n",
            )
            .stdout()
            .contains(
                " X  | X  | O  \n--------------\n O  | O  | X  \n\
                 --------------\n X  | O  | X  \n",
            )
            .stdout()
            .contains("Nobody wins the game")
            .unwrap();
    }

    #[test]
    fn computer_vs_computer() {
        assert_cli::Assert::main_binary()
            .stdin("4\n2")
            .stdout()
            .contains("Tic Tac Toe")
            .stdout()
            .contains("Select game type")
            .stdout()
            .contains(
                "1 - Human vs Human\n2 - Human vs Computer\n\
                 3 - Computer vs Human\n4 - Computer vs Computer",
            )
            .stdout()
            .contains(
                " 1  | 2  | 3  \n--------------\n 4  | 5  | 6  \n\
                 --------------\n 7  | 8  | 9  \n",
            )
            .stdout()
            .contains(
                " 1  | 2  | 3  \n--------------\n 4  | X  | 6  \n\
                 --------------\n 7  | 8  | 9  \n",
            )
            .stdout()
            .contains(
                " O  | 2  | 3  \n--------------\n 4  | X  | 6  \n\
                 --------------\n 7  | 8  | 9  \n",
            )
            .stdout()
            .contains("Nobody wins the game")
            .unwrap();
    }

    #[test]
    fn human_vs_computer() {
        assert_cli::Assert::main_binary()
            .stdin("2\n1\n2\n3\n4\n5\n6\n7\n8\n9\n2")
            .stdout()
            .contains("Tic Tac Toe")
            .stdout()
            .contains("Select game type")
            .stdout()
            .contains(
                "1 - Human vs Human\n2 - Human vs Computer\n\
                 3 - Computer vs Human\n4 - Computer vs Computer",
            )
            .stdout()
            .contains(
                " 1  | 2  | 3  \n--------------\n 4  | 5  | 6  \n\
                 --------------\n 7  | 8  | 9  \n",
            )
            .stdout()
            .contains(
                " X  | 2  | 3  \n--------------\n 4  | 5  | 6  \n\
                 --------------\n 7  | 8  | 9  \n",
            )
            .stdout()
            .contains(
                " X  | 2  | 3  \n--------------\n 4  | O  | 6  \n\
                 --------------\n 7  | 8  | 9  \n",
            )
            .stdout()
            .contains(" wins the game")
            .unwrap();
    }

    #[test]
    fn computer_vs_human() {
        assert_cli::Assert::main_binary()
            .stdin("3\n1\n2\n3\n4\n6\n7\n8\n9\n2")
            .stdout()
            .contains("Tic Tac Toe")
            .stdout()
            .contains("Select game type")
            .stdout()
            .contains(
                "1 - Human vs Human\n2 - Human vs Computer\n\
                 3 - Computer vs Human\n4 - Computer vs Computer",
            )
            .stdout()
            .contains(
                " 1  | 2  | 3  \n--------------\n 4  | 5  | 6  \n\
                 --------------\n 7  | 8  | 9  \n",
            )
            .stdout()
            .contains(
                " 1  | 2  | 3  \n--------------\n 4  | X  | 6  \n\
                 --------------\n 7  | 8  | 9  \n",
            )
            .stdout()
            .contains(
                " O  | 2  | 3  \n--------------\n 4  | X  | 6  \n\
                 --------------\n 7  | 8  | 9  \n",
            )
            .stdout()
            .contains(" wins the game")
            .unwrap();
    }
}
