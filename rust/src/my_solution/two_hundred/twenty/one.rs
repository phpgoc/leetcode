///在一个由 0 和 1 组成的二维矩阵内，找到只包含 1 的最大正方形，并返回其面积。
pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
    let mut res = 0;
    let (y, x) = match matrix.first() {
        Some(t) => (matrix.len(), t.len()),
        _ => {
            return 0;
        }
    };
    let mut dp = vec![vec![(0, 0); x]; y];
    if matrix[0][0] == '0' {
        dp[0][0] = (0, 0);
    } else {
        dp[0][0] = (1, 1);
    }
    for y_i in 1..y {
        if matrix[y_i][0] == '0' {
            dp[y_i][0] = (0, 0);
        } else {
            dp[y_i][0] = (dp[y_i - 1][0].0 + 1, 1);
        }
    }
    for x_i in 1..x {
        if matrix[0][x_i] == '0' {
            dp[0][x_i] = (0, 0);
        } else {
            dp[0][x_i] = (1, dp[0][x_i - 1].1 + 1);
        }
    }
    for y_i in 1..y {
        for x_i in 1..x {
            if matrix[y_i][x_i] == '0' {
                dp[y_i][x_i] = (0, 0);
            } else {
                dp[y_i][x_i] = (dp[y_i - 1][x_i].0 + 1, dp[y_i][x_i - 1].1 + 1);
            }
        }
    }
    for y_i in 0..y {
        if y_i + res >= y {
            break;
        }
        for x_i in 0..x {
            if x_i + res >= x {
                break;
            }
            if dp[y_i][x_i] == (0, 0) {
                continue;
            }
            let mut min = 1;
            loop {
                if x_i + min >= x || y_i + min >= y {
                    break;
                }
                if dp[y_i + min][x_i + min].0.min(dp[y_i + min][x_i + min].1) > min {
                    min += 1;
                } else {
                    break;
                }
            }
            res = res.max(min);
        }
    }

    (res * res) as i32
}
