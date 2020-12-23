///给定两个字符串形式的非负整数 num1 和num2 ，计算它们的和。
pub fn add_strings(num1: String, num2: String) -> String {
    let num1_chars = num1.chars().rev().collect::<Vec<_>>();
    let num2_chars = num2.chars().rev().collect::<Vec<_>>();
    let mut add = 0;
    let mut res = String::new();
    let mut b = 0;
    let num1_len = num1_chars.len();
    let num2_len = num2_chars.len();
    loop {
        if b < num1_len {
            add += num1_chars[b].to_string().parse::<i32>().unwrap();
        }
        if b < num2_len {
            add += num2_chars[b].to_string().parse::<i32>().unwrap();
        }
        res.push_str(&*(add % 10).to_string());
        add /= 10;
        b += 1;
        if b >= num1_len && b >= num2_len {
            break;
        }
    }
    if add == 1 {
        res.push('1');
    }
    res.chars().rev().collect()
}
