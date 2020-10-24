pub fn rob(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    match len {
        0 => return 0,
        1 => return nums[0],
        _ => {}
    }
    let mut dp = vec![0; nums.len()];
    dp[0] = nums[0];
    dp[1] = nums[0].max(nums[1]);
    for i in 2..len {
        dp[i] = dp[i - 1].max(dp[i - 2] + nums[i]);
    }
    return dp[len - 1];
}
