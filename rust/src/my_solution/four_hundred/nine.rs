use std::collections::HashSet;
///给定一个包含大写字母和小写字母的字符串，找到通过这些字母构造成的最长的回文串。
//
// 在构造过程中，请注意区分大小写。比如 "Aa" 不能当做一个回文字符串。
pub fn longest_palindrome(s: String) -> i32 {
    let mut set = HashSet::new();
    let mut res = 0;
    for i in s.chars() {
        if set.contains(&i) {
            res += 2;
            set.remove(&i);
        } else {
            set.insert(i);
        }
    }
    if !set.is_empty() {
        res += 1;
    }
    res
}
