use std::collections::HashMap;
///给定一个字符串，请将字符串里的字符按照出现的频率降序排列。
pub fn frequency_sort(s: String) -> String {
    let mut map = HashMap::new();
    let chars = s.chars().collect::<Vec<_>>();
    for &i in chars.iter() {
        let c = map.entry(i).or_insert(0);
        *c += 1;
    }
    let mut vec = map.keys().collect::<Vec<_>>();
    vec.sort_by(|b, a| map.get(a).unwrap().cmp(map.get(b).unwrap()));

    let mut res = String::new();
    for i in vec {
        for _ in 0..(*map.get(i).unwrap()) {
            res.push(*i);
        }
    }
    res
}
