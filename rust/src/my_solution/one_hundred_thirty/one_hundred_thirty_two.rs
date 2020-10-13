pub fn min_cut(s: String) -> i32 {
    let chars = s.chars().collect::<Vec<_>>();
    let len = chars.len();
    let mut dp = vec![vec![false; len]; len];
    for i in 0..len {
        dp[i][i] = true;
    }

    for i in 1..len {
        for j in 0..len - i {
            if chars[j] == chars[j + i] && (i == 1 || dp[j + 1][j + i - 1] == true) {
                dp[j][j + i] = true;
            }
        }
    }
    let mut map = vec![-1; len];
    let res = dfs(0, &dp, &len, &mut map);
    //    println!("{:?}", map);

    res
}

fn dfs(level: usize, dp: &Vec<Vec<bool>>, len: &usize, map: &mut Vec<i32>) -> i32 {
    if map[level] != -1 {
        return map[level];
    }
    let mut res = std::i32::MAX;
    if level == len - 1 {
        map[level] = 0;
        return 0;
    }
    for i in (level..*len).rev() {
        if dp[level][i] == true {
            if i == len - 1 {
                map[level] = 0;
                return 0;
            }
            res = res.min(dfs(i + 1, dp, len, map));
        }
    }
    res += 1;
    map[level] = res;
    res
}
