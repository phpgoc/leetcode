use std::collections::HashSet;

///给定两个数组，编写一个函数来计算它们的交集。
pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    nums1
        .into_iter()
        .collect::<HashSet<_>>()
        .intersection(&nums2.into_iter().collect::<HashSet<_>>())
        .cloned()
        .collect()
}
