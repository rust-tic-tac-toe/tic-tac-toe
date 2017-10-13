use std::collections::HashMap;
use board::*;
use game::*;

const INITIAL_DEPTH: i32 = 0;
const TIED: i32 = 0;
const MAX_SCORE: i32 = 1000;
const INCREMENT: i32 = 1;

pub fn find_best_space(board: Board) -> i32 {
    minimax(board, INITIAL_DEPTH, HashMap::new())
}

fn minimax(board: Board, depth: i32, mut best_score: HashMap<i32, i32>) -> i32 {
    if is_game_over(&board) {
        return score_scenarios(&board, depth)
    } else {
        for space in board.get_available_spaces().iter() {
            let emulated_board = board.clone().place_marker(*space);
            best_score.insert(*space, - minimax(emulated_board, depth + INCREMENT, HashMap::new()) );
        }

        analyse_board(best_score, depth)
    }
}

fn score_scenarios(board: &Board, depth: i32) -> i32 {
    if is_game_tied(&board) {
        return TIED
    } else {
        -MAX_SCORE / depth
    }
}

fn analyse_board(best_score: HashMap<i32, i32>, depth: i32) -> i32 {
    let space_with_highest_score: (i32, i32) = find_highest_score(best_score);
    if current_turn(depth) {
        choose_best_space(space_with_highest_score)
    } else {
        choose_highest_score(space_with_highest_score)
    }
}

fn find_highest_score(best_score: HashMap<i32, i32>) -> (i32, i32) {
    let mut scores_to_compare: Vec<(&i32, &i32)> = best_score.iter().collect();
    scores_to_compare.sort_by(|key, value| value.1.cmp(key.1));
    (*scores_to_compare[0].0, *scores_to_compare[0].1)
}

fn choose_best_space(best_space_with_score: (i32, i32)) -> i32 {
    best_space_with_score.0
}

fn choose_highest_score(best_space_with_score: (i32, i32)) -> i32 {
    best_space_with_score.1
}

fn current_turn(depth: i32) -> bool {
    depth == 0
}

pub mod tests {
    #[cfg(test)]
    use super::*;
    use board::tests::set_up_board;

    #[test]
    fn chooses_the_only_available_space() {
        let board: Board = set_up_board(3, vec![0, 1, 2, 3, 4, 8, 5, 6]);
        assert_eq!(7, find_best_space(board));
    }

    #[test]
    fn chooses_the_winning_space() {
        let board: Board = set_up_board(3, vec![0, 1, 2, 3, 4, 8]);
        assert_eq!(6, find_best_space(board));
    }

    #[test]
    fn wins_the_game() {
        let board: Board = set_up_board(3, vec![0, 4, 1, 6]);
        assert_eq!(2, find_best_space(board));
    }

    #[test]
    fn o_blocks_a_win() {
        let board: Board = set_up_board(3, vec![0, 1, 4]);
        assert_eq!(8, find_best_space(board));
    }

    #[test]
    fn x_blocks_a_win() {
        let board: Board = set_up_board(3, vec![0, 8, 6]);
        assert_eq!(3, find_best_space(board));
    }
}

