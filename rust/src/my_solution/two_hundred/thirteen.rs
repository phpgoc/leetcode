///你是一个专业的小偷，计划偷窃沿街的房屋，每间房内都藏有一定的现金。这个地方所有的房屋都 围成一圈 ，这意味着第一个房屋和最后一个房屋是紧挨着的。同时，相邻的房屋装有相互连通的防盗系统，如果两间相邻的房屋在同一晚上被小偷闯入，系统会自动报警 。
//
// 给定一个代表每个房屋存放金额的非负整数数组，计算你 在不触动警报装置的情况下 ，能够偷窃到的最高金额。
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/house-robber-ii
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn rob(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    match len {
        0 => return 0,
        1 => return nums[0],
        2 => return nums[0].max(nums[1]),
        _ => {}
    }
    let mut dp1 = vec![0; nums.len()];
    let mut dp2 = vec![0; nums.len()];
    dp1[0] = nums[0];
    dp1[1] = nums[0].max(nums[1]);
    dp1[2] = dp1[1].max(dp1[0] + nums[2]);

    dp2[1] = nums[1];
    dp2[2] = nums[2].max(nums[1]);

    for i in 3..len {
        dp1[i] = dp1[i - 1].max(dp1[i - 2] + nums[i]);
        dp2[i] = dp2[i - 1].max(dp2[i - 2] + nums[i]);
    }

    dp1[len - 2].max(dp2[len - 1])
}
