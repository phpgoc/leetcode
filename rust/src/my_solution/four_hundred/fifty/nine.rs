///给定一个非空的字符串，判断它是否可以由它的一个子串重复多次构成。给定的字符串只含有小写英文字母，并且长度不超过10000。
pub fn repeated_substring_pattern(s: String) -> bool {
    let chars = s.chars().collect::<Vec<_>>();
    let len = chars.len();
    'a: for i in 1..=len / 2 {
        if len % i != 0 {
            continue;
        }
        for j in i..len {
            if chars[j] != chars[j - i] {
                continue 'a;
            }
        }
        return true;
    }
    false
}
