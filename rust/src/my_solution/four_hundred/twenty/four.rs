use std::collections::HashMap;

///给你一个仅由大写英文字母组成的字符串，你可以将任意位置上的字符替换成另外的字符，总共可最多替换 k 次。在执行上述操作后，找到包含重复字母的最长子串的长度。
pub fn character_replacement(s: String, k: i32) -> i32 {
    let chars = s.chars().collect::<Vec<_>>();
    let mut words_position: HashMap<char, Vec<usize>> = HashMap::new();
    let k = k as usize;
    if s.len() < k {
        return s.len() as i32;
    }
    for (kk, &v) in chars.iter().enumerate() {
        let vec = words_position.entry(v).or_insert(vec![]);
        vec.push(kk);
    }
    let mut res = 0;
    for v in words_position.values() {
        let mut left = 0;
        let mut right = 0;
        while right < v.len() {
            let replace_len = v[right] - v[left] + left - right;
            if replace_len > k {
                left += 1;
            } else {
                res = res.max(v[right] - v[left] + 1 + k - replace_len);
                // dbg!((left, right, replace_len, res));
                right += 1;
            }
        }
    }
    res = res.min(s.len());
    res as i32
}
