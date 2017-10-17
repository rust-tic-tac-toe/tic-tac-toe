use board::OFFSET;
use io::*;

pub fn ask_user_to_select_space(marker: &str) -> i32 {
    ask(&select_space(marker)) - OFFSET as i32
}
