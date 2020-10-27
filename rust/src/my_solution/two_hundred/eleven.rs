use std::collections::{HashMap, HashSet};
///请你设计一个数据结构，支持 添加新单词 和 查找字符串是否与任何先前添加的字符串匹配 。
//
// 实现词典类 WordDictionary ：
//
// WordDictionary() 初始化词典对象
// void addWord(word) 将 word 添加到数据结构中，之后可以对它进行匹配
// bool search(word) 如果数据结构中存在字符串与 word 匹配，则返回 true ；否则，返回  false 。word 中可能包含一些 '.' ，每个 . 都可以表示任何一个字母。
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/design-add-and-search-words-data-structure
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
#[allow(non_snake_case)]
pub struct WordDictionary {
    data: HashMap<usize, HashSet<String>>,
}

/**
 * Your WordDictionary object will be instantiated and called as such:
 * let obj = WordDictionary::new();
 * obj.add_word(word);
 * let ret_2: bool = obj.search(word);
 */
/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {
    /** Initialize your data structure here. */
    pub fn new() -> Self {
        WordDictionary {
            data: HashMap::new(),
        }
    }

    /** Adds a word into the data structure. */
    pub fn add_word(&mut self, word: String) {
        let len = word.len();
        match self.data.get_mut(&len) {
            Some(t) => {
                t.insert(word);
            }
            None => {
                let mut set = HashSet::new();
                set.insert(word);
                self.data.insert(len, set);
            }
        }
    }

    /** Returns if the word is in the data structure. A word could contain the dot character '.' to represent any one letter. */
    pub fn search(&self, word: String) -> bool {
        match self.data.get(&word.len()) {
            Some(t) => {
                if word.contains('.') {
                    let word_char = word.chars().collect::<Vec<_>>();
                    let len = word.len();
                    'a: for j in t {
                        let t_char = j.chars().collect::<Vec<_>>();
                        for i in 0..len {
                            if word_char[i] != '.' && word_char[i] != t_char[i] {
                                continue 'a;
                            }
                        }
                        return true;
                    }
                    return false;
                } else {
                    return t.contains(&word);
                }
            }
            _ => {
                return false;
            }
        }
    }
}
