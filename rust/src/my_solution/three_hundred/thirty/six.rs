use std::collections::HashMap;
///给定一组 互不相同 的单词， 找出所有不同 的索引对(i, j)，使得列表中的两个单词， words[i] + words[j] ，可拼接成回文串。
pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
    let mut res = vec![];
    let mut map = HashMap::new();
    for k in 0..words.len() {
        map.insert(&words[k], k);
    }
    for k in 0..words.len() {
        let chars = words[k].chars().collect::<Vec<_>>();
        if chars.len() == 0 {
            for kk in 0..words.len() {
                if kk != k && is_palindrome(words[kk].clone()) {
                    res.push(vec![k as i32, kk as i32]);
                    res.push(vec![kk as i32, k as i32]);
                }
            }
            continue;
        }
        let mut reverse1 = words[k].clone().chars().rev().collect::<String>();
        let mut reverse2 = reverse1.clone();
        reverse2.pop();
        let this_str_len = chars.len();
        let mut i = 1;
        while !reverse1.is_empty() {
            match map.get(&reverse1) {
                Some(&t) => {
                    if k != t {
                        res.push(vec![k as i32, t as i32]);
                    }
                }
                None => {}
            }
            while i < this_str_len && !is_palindrome(words[k][this_str_len - i..].parse().unwrap())
            {
                reverse1.drain(..1);
                i += 1;
            }
            reverse1.drain(..1);
            i += 1;
        }
        i = 2;
        while !reverse2.is_empty() {
            match map.get(&reverse2) {
                Some(&t) => {
                    if k != t {
                        res.push(vec![t as i32, k as i32]);
                    }
                }
                None => {}
            }
            while i < this_str_len && !is_palindrome(words[k][0..i].parse().unwrap()) {
                reverse2.pop();
                i += 1;
            }
            reverse2.pop();
            i += 1;
        }
    }
    res
}
pub fn is_palindrome(x_str: String) -> bool {
    let x_reserve: String = x_str.chars().rev().collect();
    return x_str.eq(&x_reserve);
}
