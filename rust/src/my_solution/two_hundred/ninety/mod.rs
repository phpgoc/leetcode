use std::collections::HashMap;
pub mod five;
pub mod one;
#[cfg(test)]
mod tests;
///给定一种规律 pattern 和一个字符串 str ，判断 str 是否遵循相同的规律。
//
// 这里的 遵循 指完全匹配，例如， pattern 里的每个字母和字符串 str 中的每个非空单词之间存在着双向连接的对应规律。
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/word-pattern
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn word_pattern(pattern: String, s: String) -> bool {
    let pattern_char = pattern.chars().collect::<Vec<_>>();
    let s_vec = s.split(' ').collect::<Vec<_>>();
    let mut map1 = HashMap::new();
    let mut map2 = HashMap::new();
    if pattern_char.len() != s_vec.len() {
        return false;
    }
    for i in 0..pattern_char.len() {
        match map1.get(&pattern_char[i]) {
            Some(&t) => {
                if t != s_vec[i] {
                    return false;
                }
            }
            None => {
                map1.insert(pattern_char[i], s_vec[i]);
            }
        }
        match map2.get(&s_vec[i]) {
            Some(&t) => {
                if t != pattern_char[i] {
                    return false;
                }
            }
            None => {
                map2.insert(s_vec[i], pattern_char[i]);
            }
        }
    }
    true
}
