///给定一个包含 [0, n] 中 n 个数的数组 nums ，找出 [0, n] 这个范围内没有出现在数组中的那个数。
//
//
//
// 进阶：
//
// 你能否实现线性时间复杂度、仅使用额外常数空间的算法解决此问题?
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/missing-number
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn missing_number(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut nums = nums;
    nums.push(-1);
    for i in 0..n {
        while (nums[i] != i as i32 && nums[i] != -1) {
            let to = nums[i] as usize;
            nums.swap(i, to);
        }
    }
    for i in 0..=n {
        if nums[i] == -1 {
            return i as i32;
        }
    }
    unreachable!()
}
