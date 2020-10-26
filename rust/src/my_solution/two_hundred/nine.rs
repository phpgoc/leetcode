///给定一个含有 n 个正整数的数组和一个正整数 s ，找出该数组中满足其和 ≥ s 的长度最小的 连续 子数组，并返回其长度。如果不存在符合条件的子数组，返回 0。
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/minimum-size-subarray-sum
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn min_sub_array_len(s: i32, nums: Vec<i32>) -> i32 {
    let nums_len = nums.len();
    let mut cur = 0;
    let mut count = 0;
    let mut len = 0;
    for i in &nums {
        len += 1;
        count += i;
        if count >= s {
            break;
        }
    }
    if count < s {
        return 0;
    }
    while count >= s {
        count -= nums[cur];
        cur += 1;
        len -= 1;
    }
    while cur + len < nums_len {
        count = count + nums[cur + len] - nums[cur];
        while count >= s {
            len -= 1;
            count -= nums[cur + 1];
            cur += 1;
        }
        cur += 1;
    }
    len as i32 + 1
}
