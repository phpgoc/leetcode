use std::collections::HashMap;

pub fn num_decodings(s: String) -> i32 {
    let chars = s.chars().collect::<Vec<_>>();
    let len = chars.len();
    let mut map = HashMap::new();
    if len == 0 {
        return 0;
    }
    if chars[len - 1] == '0' {
        map.insert(len - 1, 0);
    } else {
        map.insert(len - 1, 1);
    }
    let res = recusive(&chars, &len, &mut map, 0);
    res
}

fn recusive(chars: &Vec<char>, len: &usize, map: &mut HashMap<usize, i32>, from: usize) -> i32 {
    if let Some(&t) = map.get(&from) {
        return t;
    }
    let mut value = 0;
    match chars[from] {
        '0' => {
            map.insert(from, 0);
            return 0;
        }
        '1' => {
            if from == len - 2 {
                value += 1;
            } else {
                value += recusive(chars, len, map, from + 2);
            }
        }
        '2' => match chars[from + 1] {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' => {
                if from == len - 2 {
                    value += 1;
                } else {
                    value += recusive(chars, len, map, from + 2);
                }
            }
            _ => {}
        },
        _ => {}
    }
    value += recusive(chars, len, map, from + 1);
    map.insert(from, value);
    return value;
}
