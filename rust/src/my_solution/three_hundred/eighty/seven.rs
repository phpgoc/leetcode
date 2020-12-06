use std::collections::HashMap;
///给定一个字符串，找到它的第一个不重复的字符，并返回它的索引。如果不存在，则返回 -1。
pub fn first_uniq_char(s: String) -> i32 {
    let mut map = HashMap::new();
    let chars = s.chars().collect::<Vec<_>>();
    for (_, &v) in chars.iter().enumerate() {
        let count = map.entry(v).or_insert(0);
        *count += 1;
    }
    for (k, &v) in chars.iter().enumerate() {
        if *map.get(&v).unwrap() == 1 {
            return k as i32;
        }
    }
    -1
}
