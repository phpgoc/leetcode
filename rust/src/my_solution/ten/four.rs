pub fn longest_palindrome(s: String) -> String {
    let len = s.len();

    if len <= 1 {
        return s;
    }
    let str_vec: Vec<char> = s.chars().collect();
    let mut matrix = vec![vec![true; len], vec![true; len]];
    let mut exist = vec![true, true];
    let mut return_str = String::from("");
    for scale in 0..len {
        let reminder = scale % 2;
        if (exist[reminder]) {
            let mut exist_in_loop = false;
            for j in 0..len - scale {
                if matrix[reminder][(j + scale / 2) as usize]
                    && str_vec.get(j) == str_vec.get(j + scale)
                {
                    if !exist_in_loop {
                        return_str = String::from(&s[j..j + scale + 1]);
                        exist_in_loop = true;
                    }
                } else {
                    matrix[reminder][j + scale / 2] = false;
                }
            }
            if !exist_in_loop {
                exist[reminder] = false;
            }
        }
    }
    return return_str;
}
