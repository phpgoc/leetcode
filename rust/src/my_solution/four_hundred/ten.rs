///给定一个非负整数数组和一个整数 m，你需要将这个数组分成 m 个非空的连续子数组。设计一个算法使得这 m 个子数组各自和的最大值最小。
//
// 注意:
pub fn split_array(nums: Vec<i32>, m: i32) -> i32 {
    let len = nums.len();
    let m = m as usize;
    let nums = nums.iter().map(|&r| r as i64).collect::<Vec<_>>();
    let mut dp = vec![vec![0; len]; m];
    dp[0][0] = nums[0];
    for i in 1..len {
        dp[0][i] = dp[0][i - 1] + nums[i];
    }
    for i in 1..m {
        for j in i..len {
            let mut min = i64::MAX;
            for l in 1..=j {
                if dp[0][j] - dp[0][j - l] >= dp[i - 1][j - l] {
                    min = min.min(dp[0][j] - dp[0][j - l]);
                    break;
                } else {
                    min = min.min(dp[i - 1][j - l]);
                }
            }
            dp[i][j] = min;
        }
    }
    dp[m - 1][len - 1] as i32
}
