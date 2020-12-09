use std::collections::HashMap;
///找到给定字符串（由小写字符组成）中的最长子串 T ， 要求 T 中的每一字符出现次数都不少于 k 。输出 T 的长度。
pub fn longest_substring(s: String, k: i32) -> i32 {
    if k == 1 {
        return s.len() as i32;
    }
    let k = k as usize;
    let mut map = HashMap::new();
    for (kk, v) in s.chars().enumerate() {
        let c = map.entry(v).or_insert(vec![]);
        c.push(kk);
    }
    let mut posible_area = vec![(0, s.len())];

    'a: loop {
        for i in map.values() {
            for j in (0..posible_area.len()).rev() {
                let this_area_len = i
                    .iter()
                    .filter(|&&x| x >= posible_area[j].0 && x < posible_area[j].1)
                    .count();
                if this_area_len > 0 && this_area_len < k {
                    let mut from = posible_area[j].0;
                    let from_for_iter = from;
                    let to = posible_area[j].1;
                    posible_area.remove(j);
                    for &p in i.iter().filter(|&&x| x >= from_for_iter && x < to) {
                        if p - from >= k {
                            posible_area.push((from, p));
                        }
                        from = p + 1;
                    }
                    if to - from >= k {
                        posible_area.push((from, to));
                    }
                    continue 'a;
                }
            }
        }
        break;
    }
    let mut res = 0;

    for i in posible_area {
        res = res.max(i.1 - i.0);
    }

    res as i32
}
