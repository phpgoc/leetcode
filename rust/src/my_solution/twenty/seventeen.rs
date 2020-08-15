pub fn letter_combinations(digits: String) -> Vec<String> {
    let chars: Vec<char> = digits.chars().collect();
    if chars.len() == 0 {
        return vec![];
    }
    let mut cur = vec![];
    let mut last = vec![String::from("")];
    for i in chars {
        cur = vec![];
        for last_string in last {
            for char in char_to_vec(i) {
                let mut clone = last_string.clone();
                clone.push(char);
                cur.push(clone);
            }
        }
        last = cur;
    }
    last
}

fn char_to_vec(chr: char) -> Vec<char> {
    match chr {
        '2' => vec!['a', 'b', 'c'],
        '3' => vec!['d', 'e', 'f'],
        '4' => vec!['g', 'h', 'i'],
        '5' => vec!['j', 'k', 'l'],
        '6' => vec!['m', 'n', 'o'],
        '7' => vec!['p', 'q', 'r', 's'],
        '8' => vec!['t', 'u', 'v'],
        '9' => vec!['w', 'x', 'y', 'z'],
        _ => vec![],
    }
}
