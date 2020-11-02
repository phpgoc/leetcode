///实现一个基本的计算器来计算一个简单的字符串表达式的值。
//
// 字符串表达式可以包含左括号 ( ，右括号 )，加号 + ，减号 -，非负整数和空格  。
pub fn calculate(s: String) -> i32 {
    let chars = s.chars().collect::<Vec<_>>();
    let mut stack: Vec<String> = vec![];
    let mut left_brace_index = vec![];

    for i in chars {
        match i {
            ' ' => {
                continue;
            }
            ')' => {
                calculate_stack(&mut stack, left_brace_index.pop().unwrap());
            }
            '(' => {
                left_brace_index.push(stack.len());
            }
            '+' | '-' => {
                stack.push(i.to_string());
            }
            _ => match stack.last_mut() {
                Some(t) => {
                    if (*t) == "+" || (*t) == "-" {
                        stack.push(i.to_string());
                    } else {
                        (*t).push(i);
                    }
                }
                None => {
                    stack.push(i.to_string());
                }
            },
        }
    }
    calculate_stack(&mut stack, 0);
    stack[0].parse::<i32>().unwrap()
}
fn calculate_stack(stack: &mut Vec<String>, from: usize) {
    let mut res = stack[from].parse::<i32>().unwrap();
    let mut i = 2;
    while from + i < stack.len() {
        let val = stack[from + i].parse::<i32>().unwrap();
        if stack[from + i - 1] == "+" {
            res += val;
        } else {
            res -= val;
        }
        i += 2;
    }
    // println!("----{:?}", stack);
    stack.drain(from..);
    stack.push(res.to_string());
    // println!("++++{:?}", stack);
}
