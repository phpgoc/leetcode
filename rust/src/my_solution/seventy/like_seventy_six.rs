use std::cmp::{Ord, Ordering};
use std::collections::{BinaryHeap, HashMap, VecDeque};

pub fn min_window(s: String, t: String) -> String {
    let s_vec: Vec<_> = s.chars().collect();
    let t_vec = t.chars().collect::<Vec<_>>();

    //t必须没重复的，如果有重复的去重下
    let mut map = HashMap::new();
    for k in 0..t_vec.len() {
        map.insert(k, VecDeque::new());
    }
    for (k, v) in s_vec.iter().enumerate() {
        match t_vec.iter().position(|&r| r == *v) {
            Some(t) => {
                map.get_mut(&t).unwrap().push_back(k);
            }
            None => {}
        }
    }

    let mut heap = BinaryHeap::new();
    let mut left = std::usize::MAX;
    let mut right = 0;
    let mut len = 0;
    for i in 0..t_vec.len() {
        let index = map.get_mut(&i).unwrap().pop_front().unwrap();
        right = right.max(index);
        left = left.min(index);
        heap.push(TupleForHeap { index, k: i });
    }
    len = right - left;
    let mut min_v = left;
    let mut max_v = right;

    loop {
        let tmp = heap.pop().unwrap();
        min_v = tmp.index;
        if max_v - min_v < len {
            len = max_v - min_v;
            left = min_v;
            right = max_v;
        }
        match map.get_mut(&tmp.k).unwrap().pop_front() {
            Some(t) => {
                heap.push(TupleForHeap { index: t, k: tmp.k });
                //                println!("heap = {:?}", heap);
                max_v = max_v.max(t);
                //                println!("min_v = {},max_v = {}", min_v, max_v);
            }
            None => break,
        }
    }
    let mut result = String::new();
    for i in left..=right {
        result.push(s_vec[i]);
    }
    result
}

#[derive(Debug, Eq, PartialEq)]
struct TupleForHeap {
    index: usize,
    k: usize,
}

impl Ord for TupleForHeap {
    fn cmp(&self, other: &Self) -> Ordering {
        other.index.cmp(&self.index)
    }
}

impl PartialOrd for TupleForHeap {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
