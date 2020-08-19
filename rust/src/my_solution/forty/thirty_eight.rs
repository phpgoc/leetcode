use std::borrow::BorrowMut;

pub fn count_and_say(n: i32) -> String {
    let mut result = String::from("1");
    if n == 1 {
        return result;
    }
    let mut i = 2;
    while i <= n {
        let chars: Vec<_> = result.chars().collect();
        let mut count = 1;
        result = String::new();
        for j in 0..chars.len() - 1 {
            if chars[j] == chars[j + 1] {
                count += 1;
            } else {
                result.push_str(format!("{}{}", count, chars[j]).borrow_mut());
                count = 1;
            }
        }
        result.push_str(format!("{}{}", count, chars[chars.len() - 1]).borrow_mut());
        i += 1;
    }
    result
}
