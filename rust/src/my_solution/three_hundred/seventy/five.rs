pub fn get_money_amount(n: i32) -> i32 {
    let n = n as usize;
    let mut dp = vec![vec![0; n + 1]; n + 1];
    for s in 1..n {
        dp[s][s + 1] = s;
    }
    for i in 2..n {
        for s in 1..=n - i {
            let mut min = std::usize::MAX;
            for m in s + i / 2..s + i {
                min = min.min(m + dp[s][m - 1].max(dp[m + 1][s + i]));
            }
            dp[s][s + i] = min;
        }
    }
    // for i in &dp {
    //     println!("{:?}", i);
    // }
    dp[1][n] as i32
}
