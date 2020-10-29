use std::collections::HashSet;
///给定一个整数数组，判断是否存在重复元素。
//
// 如果任意一值在数组中出现至少两次，函数返回 true 。如果数组中每个元素都不相同，则返回 false 。
pub fn contains_duplicate1(nums: Vec<i32>) -> bool {
    let mut set = HashSet::new();
    for i in nums {
        if set.contains(&i) {
            return true;
        } else {
            set.insert(i);
        }
    }
    false
}
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    if nums.len() < 2 {
        return false;
    }
    let mut nums = nums;
    nums.sort();
    for i in 0..=nums.len() - 2 {
        if nums[i] == nums[i + 1] {
            return true;
        }
    }
    false
}
