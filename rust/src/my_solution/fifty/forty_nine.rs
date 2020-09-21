use std::collections::HashMap;
use std::iter::FromIterator;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map: HashMap<Vec<char>, Vec<String>> = HashMap::new();
    for i in strs {
        let mut char_key: Vec<_> = i.chars().collect();
        char_key.sort();
        if map.contains_key(&char_key) {
            map.get_mut(&char_key).unwrap().push(i);
        } else {
            map.insert(char_key, vec![i]);
        }
    }
    Vec::from_iter(map.values().cloned())
}
