use std::collections::HashSet;
///给定一个字符串数组 words，找到 length(word[i]) * length(word[j]) 的最大值，并且这两个单词不含有公共字母。你可以认为每个单词只包含小写字母。如果不存在这样的两个单词，返回 0。
pub fn max_product(words: Vec<String>) -> i32 {
    let mut res = 0;
    let mut words_vec = vec![];
    for i in words {
        words_vec.push((i.chars().collect::<HashSet<_>>(), i.len()));
    }
    for i in 0..words_vec.len() {
        for j in i + 1..words_vec.len() {
            let len = words_vec[i].1 * words_vec[j].1;
            if len <= res {
                continue;
            }
            if words_vec[i]
                .0
                .intersection(&words_vec[j].0)
                .collect::<HashSet<_>>()
                .is_empty()
            {
                res = len;
            }
        }
    }

    res as i32
}
