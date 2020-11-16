///给定一个整数矩阵，找出最长递增路径的长度。
//
// 对于每个单元格，你可以往上，下，左，右四个方向移动。 你不能在对角线方向上移动或移动到边界外（即不允许环绕）。
pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
    let (x_len, y_len) = match matrix.first() {
        Some(t) => ((*t).len(), matrix.len()),
        _ => return 0,
    };
    if x_len == 0 {
        return 0;
    }
    let mut matrix_add_0 = vec![vec![0; x_len + 2]; y_len + 2];
    for y in 0..y_len {
        for x in 0..x_len {
            matrix_add_0[y + 1][x + 1] = matrix[y][x];
        }
    }
    let mut dp = vec![vec![0; x_len]; y_len];
    let mut res = 1;
    for y in 1..=y_len {
        for x in 1..=x_len {
            if dp[y - 1][x - 1] == 0 {
                res = res.max(dfs(y, x, &matrix_add_0, &mut dp));
            }
        }
    }
    // println!("{:?}", dp);
    res
}
fn dfs(y: usize, x: usize, matrix_add_0: &Vec<Vec<i32>>, dp: &mut Vec<Vec<i32>>) -> i32 {
    if dp[y - 1][x - 1] != 0 {
        return dp[y - 1][x - 1];
    }
    let mut res = 1;
    let dir = [(0, -1), (0, 1), (-1, 0), (1, 0)];
    for &i in dir.iter() {
        // println!(
        //     "y = {} x = {}, i = {:?},{:?}",
        //     y,
        //     x,
        //     i,
        //     matrix_add_0[(y as i32 + i.0) as usize][(x as i32 + i.1) as usize]
        // );

        if matrix_add_0[(y as i32 + i.0) as usize][(x as i32 + i.1) as usize] > matrix_add_0[y][x] {
            res = res.max(
                dfs(
                    (y as i32 + i.0) as usize,
                    (x as i32 + i.1) as usize,
                    matrix_add_0,
                    dp,
                ) + 1,
            );
        }
    }
    dp[y - 1][x - 1] = res;
    res
}
