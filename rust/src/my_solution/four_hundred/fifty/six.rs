use std::collections::BTreeSet;

///给定一个整数序列：a1, a2, ..., an，一个132模式的子序列 ai, aj, ak 被定义为：当 i < j < k 时，ai < ak < aj。设计一个算法，当给定有 n 个数字的序列时，验证这个序列中是否含有132模式的子序列。
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/132-pattern
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn find132pattern(nums: Vec<i32>) -> bool {
    let len = nums.len();
    let mut left = vec![None; len];
    let mut right = vec![None; len];
    let mut min_left = i32::MAX;
    let mut btreesetfor_right = BTreeSet::new();
    for i in 0..len {
        if nums[i] > min_left {
            left[i] = Some(min_left);
        } else {
            min_left = nums[i];
        }
        if let Some(&t) = btreesetfor_right.range(..nums[len - 1 - i]).next_back() {
            right[len - i - 1] = Some(t);
        }
        btreesetfor_right.insert(nums[len - 1 - i]);
    }

    for i in 0..len {
        match (left[i], right[i]) {
            (Some(a), Some(b)) => {
                if a < b {
                    return true;
                }
            }
            _ => {}
        }
    }
    false
}
