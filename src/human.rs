use io;
use board;

pub fn ask_user_to_select_space() -> i32 {
    io::select_number() - board::OFFSET as i32
}
