///给定一个由正整数组成且不存在重复数字的数组，找出和为给定目标正整数的组合的个数。
pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
    let mut dp = vec![0; target as usize + 1];
    dp[0] = 1;
    for i in 1..=target as usize {
        for &j in nums.iter() {
            if i >= j as usize {
                dp[i] += dp[i - j as usize];
            }
        }
    }
    dp[target as usize]
}
