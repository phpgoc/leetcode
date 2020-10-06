pub fn restore_ip_addresses(s: String) -> Vec<String> {
    let chars = s.chars().collect::<Vec<_>>();
    let mut res = vec![];
    let len = chars.len();
    let mut str = String::new();
    recursive(&chars, &len, 0, &mut str, &mut res, 4);
    //    println!("{:?}", res);
    res
}

fn recursive(
    chars: &Vec<char>,
    len: &usize,
    from: usize,
    str: &mut String,
    res: &mut Vec<String>,
    level: usize,
) {
    if len - from > level * 3 || len - from < level {
        return;
    }
    if level == 1 {
        //        println!("res = {:?},from ={}, str = {}", res, from, str);

        if len - from == 3 {
            match chars[from] {
                '3' | '4' | '5' | '6' | '7' | '8' | '9' => return,
                '2' => match chars[from + 1] {
                    '6' | '7' | '8' | '9' => return,
                    '5' => match chars[from + 2] {
                        '6' | '7' | '8' | '9' => return,
                        _ => {}
                    },
                    _ => {}
                },
                _ => {}
            }
        }
        if len - from != 1 && chars[from] == '0' {
            return;
        }
        for i in from..*len {
            str.push(chars[i]);
        }
        res.push((*str).clone());
        for _ in from..*len {
            str.pop();
        }
        return;
    }
    str.push(chars[from]);
    str.push('.');
    recursive(chars, len, from + 1, str, res, level - 1);
    str.pop();
    str.pop();
    if chars[from] == '0' {
        return;
    }
    if from + level + 1 <= *len && from + 3 * level >= *len + 1 {
        str.push(chars[from]);
        str.push(chars[from + 1]);
        str.push('.');
        recursive(chars, len, from + 2, str, res, level - 1);
        str.pop();
        str.pop();
        str.pop();
    }
    if from + level + 2 <= *len && from + 3 * level >= *len {
        match chars[from] {
            '3' | '4' | '5' | '6' | '7' | '8' | '9' => return,
            '2' => match chars[from + 1] {
                '6' | '7' | '8' | '9' => return,
                '5' => match chars[from + 2] {
                    '6' | '7' | '8' | '9' => return,
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }
        str.push(chars[from]);
        str.push(chars[from + 1]);
        str.push(chars[from + 2]);
        str.push('.');
        recursive(chars, len, from + 3, str, res, level - 1);
        str.pop();
        str.pop();
        str.pop();
        str.pop();
    }
}
