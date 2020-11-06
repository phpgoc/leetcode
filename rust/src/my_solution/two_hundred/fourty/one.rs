///给定一个含有数字和运算符的字符串，为表达式添加括号，改变其运算优先级以求出不同的结果。你需要给出所有可能的组合的结果。有效的运算符号包含 +, - 以及 * 。
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/different-ways-to-add-parentheses
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn diff_ways_to_compute(input: String) -> Vec<i32> {
    let chars = input.chars().collect::<Vec<_>>();

    let mut cur_number_str = String::new();
    let mut stack_number = vec![];
    let mut stack_symbol = vec![]; // * - /
    for i in &chars {
        match i {
            '0'..='9' => {
                cur_number_str.push(*i);
            }
            '+' | '-' | '*' => {
                stack_number.push(cur_number_str.parse::<i32>().unwrap());
                cur_number_str.clear();
                stack_symbol.push(*i);
            }
            _ => {
                //blank or other
                continue;
            }
        }
    }
    stack_number.push(cur_number_str.parse::<i32>().unwrap());
    let number_len = stack_number.len();
    let mut memory = vec![vec![None; number_len]; number_len];
    for i in 0..number_len {
        memory[i][i] = Some(vec![stack_number[i]]);
    }
    recursive(0, number_len - 1, &stack_number, &stack_symbol, &mut memory);
    memory[0][number_len - 1].as_ref().unwrap().to_vec()
}

fn recursive(
    from: usize,
    to: usize, //not include
    stack_number: &Vec<i32>,
    stack_symbol: &Vec<char>,
    memory: &mut Vec<Vec<Option<Vec<i32>>>>,
) {
    if from == to {
        return;
    }
    let mut res = vec![];
    for i in from..to {
        if memory[from][i].is_none() {
            recursive(from, i, stack_number, stack_symbol, memory);
        }
        if memory[i + 1][to].is_none() {
            recursive(i + 1, to, stack_number, stack_symbol, memory);
        }

        for &j1 in memory[from][i].as_ref().unwrap() {
            for &j2 in memory[i + 1][to].as_ref().unwrap() {
                match stack_symbol[i] {
                    '*' => {
                        res.push(j1 * j2);
                    }
                    '+' => {
                        res.push(j1 + j2);
                    }
                    '-' => {
                        res.push(j1 - j2);
                    }
                    _ => panic!("error"),
                }
            }
        }
    }
    memory[from][to] = Some(res);
}
