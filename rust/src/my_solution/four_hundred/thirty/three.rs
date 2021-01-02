use std::collections::{HashMap, VecDeque};

///一条基因序列由一个带有8个字符的字符串表示，其中每个字符都属于 "A", "C", "G", "T"中的任意一个。
//
// 假设我们要调查一个基因序列的变化。一次基因变化意味着这个基因序列中的一个字符发生了变化。
//
// 例如，基因序列由"AACCGGTT" 变化至 "AACCGGTA" 即发生了一次基因变化。
//
// 与此同时，每一次基因变化的结果，都需要是一个合法的基因串，即该结果属于一个基因库。
//
// 现在给定3个参数 — start, end, bank，分别代表起始基因序列，目标基因序列及基因库，请找出能够使起始基因序列变化为目标基因序列所需的最少变化次数。如果无法实现目标变化，请返回 -1。
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/minimum-genetic-mutation
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn min_mutation(start: String, end: String, bank: Vec<String>) -> i32 {
    let mut map = HashMap::new();
    let start_chars = start.chars().collect::<Vec<_>>();
    let end_chars = end.chars().collect::<Vec<_>>();
    let bank_chars = bank
        .iter()
        .map(|r| r.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut target = usize::MAX;
    for i in 0..bank_chars.len() {
        if diff_two_chars(&bank_chars[i], &start_chars) == 1 {
            let vec = map.entry(usize::MAX - 1).or_insert(vec![]);
            vec.push(i);
        }
        let diff_end = diff_two_chars(&bank_chars[i], &end_chars);
        if diff_end == 1 {
            let vec = map.entry(i).or_insert(vec![]);
            vec.push(usize::MAX);
        } else if diff_end == 0 {
            target = i;
        }

        for j in i + 1..bank_chars.len() {
            if diff_two_chars(&bank_chars[i], &bank_chars[j]) == 1 {
                let vec = map.entry(i).or_insert(vec![]);
                vec.push(j);
                let vec = map.entry(j).or_insert(vec![]);
                vec.push(i);
            }
        }
    }
    if target == usize::MAX {
        return -1;
    }
    let mut queue = VecDeque::new();
    queue.push_back(vec![usize::MAX - 1]);
    while let Some(first) = queue.pop_front() {
        // println!("{:?}", queue);
        let last = first.last().unwrap();
        if let Some(t) = map.get(last) {
            if t.contains(&target) {
                return first.len() as i32;
            }
            for i in t {
                if !first.contains(i) {
                    let mut new_trace = first.clone();
                    new_trace.push(*i);
                    queue.push_back(new_trace);
                }
            }
        }
    }

    -1
}
fn diff_two_chars(chars1: &Vec<char>, char2: &Vec<char>) -> i32 {
    if chars1.len() != char2.len() {
        return 2;
    }
    let mut c = 0;
    for i in 0..chars1.len() {
        if chars1[i] != char2[i] {
            c += 1;
            if c != 1 {
                return 2;
            }
        }
    }
    c
}
