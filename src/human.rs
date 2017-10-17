use io;

const OFFSET: usize = 1;

pub fn ask_user_to_select_space() -> i32 {
    io::select_number() - OFFSET as i32
}
