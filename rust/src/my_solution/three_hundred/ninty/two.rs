use lazy_static::lazy_static;
use std::collections::{BTreeSet, HashMap};
use std::sync::Mutex;

lazy_static! {
    static ref MAP: Mutex<HashMap<String, HashMap<char, BTreeSet<usize>>>> =
        Mutex::new(HashMap::new());
}
///给定字符串 s 和 t ，判断 s 是否为 t 的子序列。
//
// 字符串的一个子序列是原始字符串删除一些（也可以不删除）字符而不改变剩余字符相对位置形成的新字符串。（例如，"ace"是"abcde"的一个子序列，而"aec"不是）。
//
// 进阶：
//
// 如果有大量输入的 S，称作 S1, S2, ... , Sk 其中 k >= 10亿，你需要依次检查它们是否为 T 的子序列。在这种情况下，你会怎样改变代码？
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/is-subsequence
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn is_subsequence(s: String, t: String) -> bool {
    let mut map = MAP.lock().unwrap();
    if !map.contains_key(&t) {
        let mut k_v_map = HashMap::new();
        for (k, v) in t.chars().enumerate() {
            let v_set = k_v_map.entry(v).or_insert(BTreeSet::new());
            v_set.insert(k + 1);
        }
        map.insert(t.clone(), k_v_map);
    }
    let this_t_map = map.get(&t).unwrap();
    let mut last = 0;
    for i in s.chars() {
        if let Some(t) = this_t_map.get(&i) {
            if let Some(v) = t.range(last..).next() {
                last = v + 1;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}
