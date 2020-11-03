pub mod four;
pub mod one;
#[cfg(test)]
mod tests;
pub mod three;
pub mod seven;
pub mod eight;
///在整数数组 nums 中，是否存在两个下标 i 和 j，使得 nums [i] 和 nums [j] 的差的绝对值小于等于 t ，且满足 i 和 j 的差的绝对值也小于等于 ķ 。
//
// 如果存在则返回 true，不存在返回 false。
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/contains-duplicate-iii
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
    use std::collections::HashSet;
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
    if t == 0 {
        return contains_nearby_duplicate(nums, k);
    }
    let len = nums.len();
    let k = k as usize;
    let t = t as i64;
    let nums: Vec<i64> = nums.iter().map(|i| *i as i64).collect();
    for i in 1..=k {
        for j in 0..len - i {
            if (nums[j] - nums[j + i]).abs() <= t {
                return true;
            }
        }
    }
    false
}
