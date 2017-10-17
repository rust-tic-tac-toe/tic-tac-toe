use board::OFFSET;
use io::*;

pub fn ask_user_to_select_space() -> i32 {
    select_number() - OFFSET as i32
}
