///给定一个仅包含数字 0-9 的字符串和一个目标值，在数字之间添加二元运算符（不是一元）+、- 或 * ，返回所有能够得到目标值的表达式。
pub fn add_operators(num: String, target: i32) -> Vec<String> {
    let mut res = vec![];
    let len = num.len();
    if len == 0 {
        return res;
    }
    let mut str = num;
    dfs(1, len, target, &mut str, &mut res, 0);
    res
}
fn dfs(
    from: usize,
    len: usize,
    target: i32,
    str: &mut String,
    res: &mut Vec<String>,
    add_bit: usize,
) {
    if from == len {
        if calculate(str.clone()) == Some(target) {
            res.push(str.clone());
        }
        return;
    }

    dfs(from + 1, len, target, str, res, add_bit);
    for &i in ['*', '+', '-'].iter() {
        str.insert(from + add_bit, i);
        dfs(from + 1, len, target, str, res, add_bit + 1);
        str.remove(from + add_bit);
    }
}
pub fn calculate(s: String) -> Option<i32> {
    let chars = s.chars().collect::<Vec<_>>();
    let mut cur_number_str = String::new();
    let mut stack_number = vec![];
    let mut stack_symbol = vec![]; // * - /
    for i in &chars {
        match i {
            '0'..='9' => {
                cur_number_str.push(*i);
            }
            '+' | '-' | '*' => {
                if cur_number_str.starts_with('0') && cur_number_str.len() > 1 {
                    return None;
                }
                match cur_number_str.parse::<i32>() {
                    Ok(t) => {
                        stack_number.push(t);
                    }
                    _ => {
                        return None;
                    }
                }
                cur_number_str.clear();
                stack_symbol.push(*i);
            }
            _ => {
                continue;
            }
        }
    }
    if cur_number_str.starts_with('0') && cur_number_str.len() > 1 {
        return None;
    }
    match cur_number_str.parse::<i32>() {
        Ok(t) => {
            stack_number.push(t);
        }
        _ => {
            return None;
        }
    }

    let mut map = vec![];
    for i in 0..chars.len() {
        map.push(i);
    }
    let mut val = 0;
    for (k, v) in stack_symbol.iter().enumerate() {
        match v {
            '*' => {
                if (stack_number[k] as i64 * stack_number[k + 1] as i64) > std::i32::MAX as i64 {
                    return None;
                }
                val = stack_number[k] * stack_number[k + 1];
                map[k + 1] = map[k];
                stack_number[k + 1] = val;
                stack_number[map[k]] = val;
            }
            _ => {}
        }
    }
    let mut res = stack_number[0];
    for (k, v) in stack_symbol.iter().enumerate() {
        match v {
            '+' => {
                res += stack_number[k + 1];
            }
            '-' => {
                res -= stack_number[k + 1];
            }
            _ => {}
        }
    }
    Some(res)
}
