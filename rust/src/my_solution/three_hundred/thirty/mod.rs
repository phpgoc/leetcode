pub mod eight;
pub mod four;
pub mod six;
#[cfg(test)]
mod tests;
pub mod two;
///给定一个已排序的正整数数组 nums，和一个正整数 n 。从 [1, n] 区间内选取任意个数字补充到 nums 中，使得 [1, n] 区间内的任何数字都可以用 nums 中某几个数字的和来表示。请输出满足上述要求的最少需要补充的数字个数。
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/patching-array
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
    let mut i = 0;
    let mut patch = 0;
    let mut miss = 1;
    while miss <= n {
        if i < nums.len() && nums[i] <= miss {
            miss += nums[i];
            i += 1;
        } else {
            patch += 1;
            if miss > n / 2 {
                break;
            }
            miss += miss;
        }
    }
    patch
}
