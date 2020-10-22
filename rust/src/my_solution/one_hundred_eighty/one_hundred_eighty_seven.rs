use std::collections::HashSet;
use std::iter::FromIterator;

pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
    let chars = s.chars().collect::<Vec<_>>();
    let len = chars.len();
    let mut seen = HashSet::new();
    if len <= 10 {
        return vec![];
    }
    let mut res_set = HashSet::new();
    for i in 0..=len - 10 {
        let str = String::from(&s[i..i + 10]);
        println!("{:?}", str);

        if seen.contains(&str) {
            res_set.insert(str);
        } else {
            seen.insert(str);
        }
    }

    Vec::from_iter(res_set)
}
