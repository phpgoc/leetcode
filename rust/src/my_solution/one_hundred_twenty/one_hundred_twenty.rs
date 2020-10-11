pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
    if triangle.len() == 0 {
        return 0;
    }
    let mut dp = triangle[0].clone();
    for i in 1..triangle.len() {
        let mut last = dp[0];
        let mut min = dp[0];
        dp[0] += triangle[i][0];
        for j in 1..i {
            min = last.min(dp[j]);
            last = dp[j];
            dp[j] = min + triangle[i][j];
        }
        dp.push(last + triangle[i][i]);
    }

    let mut res = dp[0];
    for i in dp {
        res = res.min(i);
    }

    res
}
