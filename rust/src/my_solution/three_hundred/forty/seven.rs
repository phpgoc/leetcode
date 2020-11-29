use std::cmp::{Ord, Ordering};
use std::collections::{BinaryHeap, HashMap};
///给定一个非空的整数数组，返回其中出现频率前 k 高的元素。
pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    for i in nums {
        let counter = map.entry(i).or_insert(0);
        *counter += 1;
    }
    let mut heap = BinaryHeap::new();
    for i in map {
        heap.push(TupleForHeap {
            val: i.0,
            count: i.1,
        });
        if heap.len() > k as usize {
            heap.pop();
        }
    }
    let mut res = vec![];
    while let Some(t) = heap.pop() {
        res.push(t.val);
    }
    res
}

#[derive(Debug, Eq, PartialEq)]
struct TupleForHeap {
    val: i32,
    count: i32,
}

impl Ord for TupleForHeap {
    fn cmp(&self, other: &Self) -> Ordering {
        other.count.cmp(&self.count)
    }
}

impl PartialOrd for TupleForHeap {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
