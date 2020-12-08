use std::collections::HashMap;
///给定两个字符串 s 和 t，它们只包含小写字母。
//
// 字符串 t 由字符串 s 随机重排，然后在随机位置添加一个字母。
//
// 请找出在 t 中被添加的字母。
pub fn find_the_difference(s: String, t: String) -> char {
    let mut map = HashMap::new();
    let s_chars = s.chars().collect::<Vec<_>>();
    let t_chars = t.chars().collect::<Vec<_>>();
    for i in s_chars {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }
    for i in t_chars {
        let count = map.entry(i).or_insert(0);
        *count -= 1;
    }
    for (k, &v) in map.iter() {
        if v == -1 {
            return *k;
        }
    }
    unreachable!();
}
