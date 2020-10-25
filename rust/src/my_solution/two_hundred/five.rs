use std::collections::HashMap;

///给定两个字符串 s 和 t，判断它们是否是同构的。
//
// 如果 s 中的字符可以被替换得到 t ，那么这两个字符串是同构的。
//
// 所有出现的字符都必须用另一个字符替换，同时保留字符的顺序。两个字符不能映射到同一个字符上，但字符可以映射自己本身。
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/isomorphic-strings
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn is_isomorphic(s: String, t: String) -> bool {
    let mut map1 = HashMap::new();
    let mut map2 = HashMap::new();
    let len = s.len();
    if t.len() != len {
        return false;
    }
    let s_vec = s.chars().collect::<Vec<_>>();
    let t_vec = t.chars().collect::<Vec<_>>();
    for i in 0..len {
        match map1.get(&s_vec[i]) {
            Some(&mt) => {
                if mt != t_vec[i] {
                    return false;
                }
            }
            _ => {
                map1.insert(s_vec[i], t_vec[i]);
            }
        }
        match map2.get(&t_vec[i]) {
            Some(&mt) => {
                if mt != s_vec[i] {
                    return false;
                }
            }
            _ => {
                map2.insert(t_vec[i], s_vec[i]);
            }
        }
    }
    true
}
