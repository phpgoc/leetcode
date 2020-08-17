pub fn my_atoi(str: String) -> i32 {
    let str_trim = String::from(str.trim());
    if str_trim.len() == 0 {
        return 0;
    }
    let mut char_src: Vec<char> = str_trim.chars().collect();
    let mut char_dst = vec![];
    let mut minus = false;
    if char_src[0] == '+' {
        char_src.remove(0);
    } else if char_src[0] == '-' {
        char_src.remove(0);
        char_dst.push('-');
        minus = true;
    }
    if char_src.len() == 0 || !char_src[0].is_numeric() {
        return 0;
    }
    for i in char_src {
        if (i.is_numeric()) {
            char_dst.push(i);
        } else {
            break;
        }
    }
    let return_str: String = char_dst.into_iter().collect();
    let r: i32 = match return_str.parse() {
        Ok(T) => T,
        Err(_) => {
            if minus {
                std::i32::MIN
            } else {
                std::i32::MAX
            }
        }
    };
    r
}
