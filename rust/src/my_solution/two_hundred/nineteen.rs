use std::collections::HashSet;
///给定一个整数数组和一个整数 k，判断数组中是否存在两个不同的索引 i 和 j，使得 nums [i] = nums [j]，并且 i 和 j 的差的 绝对值 至多为 k。
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/contains-duplicate-ii
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    let len = nums.len();
    if len < 2 {
        return false;
    }
    let k = k as usize;
    let mut set = HashSet::new();
    for i in 0..k {
        if set.contains(&nums[i]) {
            return true;
        } else {
            set.insert(nums[i]);
        }
    }
    for i in k..len {
        if set.contains(&nums[i]) {
            return true;
        } else {
            set.insert(nums[i]);
        }
        set.remove(&nums[i - k]);
    }
    false
}
