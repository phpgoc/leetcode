pub fn remove_kdigits(num: String, k: i32) -> String {
    let mut remove_set = vec![];
    let k = k as usize;
    let mut chars = num.chars().collect::<Vec<_>>();
    let mut stack: Vec<(usize, char)> = vec![];
    let mut count = 0;
    'a: for (i, &v) in chars.iter().enumerate() {
        // println!("i = {},v = {},remove_set {:?}", i, v, remove_set);

        if v == '0' {
            let stack_len = stack.len();
            if stack_len + count < k {
                remove_set.extend(stack.iter().map(|r| r.0));
                remove_set.push(i);
                count += stack_len;
                stack.clear();
            } else if stack_len + count == k {
                if stack_len != 0 {
                    remove_set.extend(stack[stack_len + count - k..].iter().map(|r| r.0));
                }
                remove_set.push(i);
                count = k;
            } else {
                remove_set.extend(stack[stack_len + count - k..].iter().map(|r| r.0));
                count = k;
                break;
            }
        } else {
            if count == k {
                break;
            }
            if stack.is_empty() {
                stack.push((i, v));
            } else {
                while !stack.is_empty() {
                    if stack.last().unwrap().1 > v {
                        remove_set.push(stack.pop().unwrap().0);
                        count += 1;
                        if count == k {
                            break 'a;
                        }
                    } else {
                        break;
                    }
                }
                stack.push((i, v));
            }
        }
    }
    if remove_set.len() == chars.len() {
        return String::from("0");
    }
    if count != k {
        if stack.len() + count < k {
            return String::from("0");
        }
        remove_set.extend(stack[stack.len() + count - k..].iter().map(|r| r.0));
    }
    remove_set.sort();

    for i in remove_set.iter().rev() {
        chars.remove(*i);
    }
    chars.iter().collect()
}
