use std::collections::HashMap;
use board::*;
use game::*;
use players::*;

const INITIAL_DEPTH: i32 = 0;

pub fn find_best_space(player: &Players, board: Board) -> i32 {
    let best_score: HashMap<i32, i32> = HashMap::new();
    minimax(board, INITIAL_DEPTH, best_score)
}

fn minimax(board: Board, depth: i32, mut best_score: HashMap<i32, i32>) -> i32 {
    if is_game_tied(&board) {
        return 0
    } else if is_game_over(&board) {
        return -1000 / depth
    } else {
        for space in board.get_available_spaces().iter() {
            let fake_board = board.clone().place_marker(*space);
            let new_best_score: HashMap<i32, i32> = HashMap::new();
            best_score.insert(*space, -1 * minimax(fake_board, depth + 1, new_best_score) );
        }

        let mut comparable: Vec<(&i32, &i32)> = best_score.iter().collect();
        comparable.sort_by(|key, value| value.1.cmp(key.1));
        if depth == 0 {
            *comparable[0].0
        } else {
            *comparable[0].1
        }
    }

}

pub mod tests {
    #[cfg(test)]
    use super::*;
    use board::tests::set_up_board;

    #[test]
    fn chooses_the_only_available_space() {
        let board: Board = set_up_board(3, vec![0, 1, 2, 3, 4, 8, 5, 6]);
        let player = Players::Computer { marker: "X".to_string() };
        assert_eq!(7, find_best_space(&player, board));
    }

    #[test]
    fn chooses_the_winning_space() {
        let board: Board = set_up_board(3, vec![0, 1, 2, 3, 4, 8]);
        let player = Players::Computer { marker: "X".to_string() };
        assert_eq!(6, find_best_space(&player, board));
    }

    #[test]
    fn wins_the_game() {
        let board: Board = set_up_board(3, vec![0, 4, 1, 6]);
        let player = Players::Computer { marker: "X".to_string() };
        assert_eq!(2, find_best_space(&player, board));
    }

    #[test]
    fn o_blocks_a_win() {
        let board: Board = set_up_board(3, vec![0, 1, 4]);
        let player = Players::Computer { marker: "X".to_string() };
        assert_eq!(8, find_best_space(&player, board));
    }

    #[test]
    fn x_blocks_a_win() {
        let board: Board = set_up_board(3, vec![0, 8, 6]);
        let player = Players::Computer { marker: "X".to_string() };
        assert_eq!(3, find_best_space(&player, board));
    }
}

