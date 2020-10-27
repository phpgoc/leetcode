///给定一个字符串 s，你可以通过在字符串前面添加字符将其转换为回文串。找到并返回可以用这种方式转换的最短回文串。
pub fn shortest_palindrome(s: String) -> String {
    let len = s.len();
    if len == 0 {
        return s;
    }
    let chars = s.chars().collect::<Vec<_>>();
    let mut cur_start = 0;
    let mut end = len - 1;
    let mut cur_end = len - 1;
    while cur_start < cur_end {
        if chars[cur_start] == chars[cur_end] {
            cur_start += 1;
            cur_end -= 1;
        } else {
            cur_start = 0;
            end -= 1;
            cur_end = end;
        }
    }
    format!("{}{}", &s[end + 1..].chars().rev().collect::<String>(), s)
}
