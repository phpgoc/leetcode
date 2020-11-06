///给定两个字符串 s 和 t ，编写一个函数来判断 t 是否是 s 的字母异位词。
pub fn is_anagram(s: String, t: String) -> bool {
    let mut s = s.chars().collect::<Vec<_>>();
    s.sort();
    let mut t = t.chars().collect::<Vec<_>>();
    t.sort();
    s == t
}
