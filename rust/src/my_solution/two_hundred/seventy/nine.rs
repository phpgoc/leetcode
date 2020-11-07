use std::collections::HashMap;

///给定正整数 n，找到若干个完全平方数（比如 1, 4, 9, 16, ...）使得它们的和等于 n。你需要让组成和的完全平方数的个数最少。
pub fn num_squares(n: i32) -> i32 {
    let mut i = 1;
    let mut square_nums = vec![];
    while i * i < n {
        square_nums.push((i * i) as usize);
        i += 1;
    }
    if i * i == n {
        return 1;
    }
    let n = n as usize;
    let mut dp = vec![1; n];
    for i in 0..n {
        if square_nums.contains(&(i + 1)) {
            continue;
        }
        let mut res = std::i32::MAX;
        for &j in &square_nums {
            if j > i {
                break;
            }
            res = res.min(dp[i - j]);
            if res == 1 {
                break;
            }
        }
        dp[i] = res + 1;
    }
    dp[n - 1]
}
pub fn num_squares2(n: i32) -> i32 {
    let mut memory = HashMap::new();
    recursive(n, &mut memory)
}
fn recursive(n: i32, memory: &mut HashMap<i32, i32>) -> i32 {
    match memory.get(&n) {
        Some(&t) => {
            return t;
        }
        None => {}
    }
    let mut res = std::i32::MAX;
    let mut i = 1;
    while i * i < n {
        i += 1;
    }
    if i * i == n {
        memory.insert(n, 1);
        return 1;
    }
    while i > 1 {
        i -= 1;
        res = res.min(recursive(n - i * i, memory));
    }
    memory.insert(n, res + 1);
    res + 1
}
