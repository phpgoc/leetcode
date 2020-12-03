use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

///给定两个以升序排列的整形数组 nums1 和 nums2, 以及一个整数 k。
//
// 定义一对值 (u,v)，其中第一个元素来自 nums1，第二个元素来自 nums2。
//
// 找到和最小的 k 对数字 (u1,v1), (u2,v2) ... (uk,vk)。
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/find-k-pairs-with-smallest-sums
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, mut k: i32) -> Vec<Vec<i32>> {
    let mut res = vec![];
    if nums1.is_empty() || nums2.is_empty() {
        return res;
    }

    let mut heap = BinaryHeap::new();
    let mut set = HashSet::new();
    heap.push(TupleForHeap {
        i1: 0,
        i2: 0,
        n1: nums1[0],
        n2: nums2[0],
    });
    while let Some(cur) = heap.pop() {
        if k == 0 {
            break;
        }
        res.push(vec![cur.n1, cur.n2]);
        if cur.i1 != nums1.len() - 1 && !set.contains(&(cur.i1 + 1, cur.i2)) {
            heap.push(TupleForHeap {
                i1: cur.i1 + 1,
                i2: cur.i2,
                n1: nums1[cur.i1 + 1],
                n2: cur.n2,
            });
            set.insert((cur.i1 + 1, cur.i2));
        }
        if cur.i2 != nums2.len() - 1 && !set.contains(&(cur.i1, cur.i2 + 1)) {
            heap.push(TupleForHeap {
                i1: cur.i1,
                i2: cur.i2 + 1,
                n1: cur.n1,
                n2: nums2[cur.i2 + 1],
            });
            set.insert((cur.i1, cur.i2 + 1));
        }

        k -= 1;
    }
    res
}
#[derive(Debug, Eq, PartialEq)]
struct TupleForHeap {
    i1: usize,
    i2: usize,
    n1: i32,
    n2: i32,
}

impl Ord for TupleForHeap {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.n1 + other.n2).cmp(&(self.n1 + self.n2))
    }
}

impl PartialOrd for TupleForHeap {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
