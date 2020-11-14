///给你一个字符串 s ，请你去除字符串中重复的字母，使得每个字母只出现一次。需保证 返回结果的字典序最小（要求不能打乱其他字符的相对位置）。
pub fn remove_duplicate_letters(s: String) -> String {
    let mut stack: Vec<char> = vec![];
    let chars = s.chars().collect::<Vec<_>>();
    for (k, &v) in chars.iter().enumerate() {
        if stack.contains(&v) {
            continue;
        }
        while !stack.is_empty() && stack.last().unwrap() > &v {
            if s[k..].find(stack[stack.len() - 1]).is_some() {
                stack.pop();
            } else {
                break;
            }
        }
        stack.push(v);
    }

    stack.iter().collect()
}
