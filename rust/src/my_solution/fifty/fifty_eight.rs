pub fn length_of_last_word(s: String) -> i32 {
    let chars: Vec<_> = s.chars().collect();
    let mut started = false;
    let mut len = 0;
    for v in chars.into_iter().rev() {
        if v == ' ' {
            if started {
                break;
            } else {
                continue;
            }
        } else {
            started = true;
            len += 1;
        }
    }
    len
}
