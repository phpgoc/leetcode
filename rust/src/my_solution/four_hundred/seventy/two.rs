use std::collections::{BTreeSet, HashMap, HashSet};

///给定一个不含重复单词的列表，编写一个程序，返回给定单词列表中所有的连接词。
//
// 连接词的定义为：一个字符串完全是由至少两个给定数组中的单词组成的。
pub fn find_all_concatenated_words_in_a_dict(words: Vec<String>) -> Vec<String> {
    let tree = words.iter().filter(|r| *r != "").collect::<BTreeSet<_>>();
    let mut res = vec![];
    let mut memory = HashMap::new();
    for &i in tree.iter() {
        memory.insert(i, HashSet::new());
        if dfs(0, i, &tree, &mut memory) {
            res.push(i.clone());
        }
    }
    // println!(
    //     "{:?}",
    //     memory
    //         .iter()
    //         .filter(|(a, r)| !r.is_empty())
    //         .collect::<HashMap<_, _>>()
    //         .keys()
    // );

    res
}
fn dfs(
    position: usize,
    str: &String,
    tree: &BTreeSet<&String>,
    memory: &mut HashMap<&String, HashSet<usize>>,
) -> bool {
    if memory.get(str).unwrap().contains(&position) {
        return false;
    }
    if position != 0 && tree.contains(&str[position..].to_string()) {
        return true;
    }

    for i in tree
        .range((str[position..=position].to_string())..(str[position..].to_string()))
        .rev()
    {
        if str[position..].starts_with(*i) && dfs(position + i.len(), str, tree, memory) {
            return true;
        }
    }
    memory.get_mut(str).unwrap().insert(position);
    false
}
