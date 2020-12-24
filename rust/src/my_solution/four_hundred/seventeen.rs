///给定一个 m x n 的非负整数矩阵来表示一片大陆上各个单元格的高度。“太平洋”处于大陆的左边界和上边界，而“大西洋”处于大陆的右边界和下边界。
//
// 规定水流只能按照上、下、左、右四个方向流动，且只能从高到低或者在同等高度上流动。
//
// 请找出那些水流既可以流动到“太平洋”，又能流动到“大西洋”的陆地单元的坐标。
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/pacific-atlantic-water-flow
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn pacific_atlantic(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let (x_len, y_len) = match matrix.first() {
        Some(t) => ((*t).len(), matrix.len()),
        _ => return vec![],
    };
    if x_len == 0 {
        return vec![];
    }
    let mut dp = vec![vec![None; x_len]; y_len];
    dp[0][x_len - 1] = Some((true, true));
    dp[y_len - 1][0] = Some((true, true));
    let mut trace = vec![];
    for y in 0..y_len {
        for x in 0..x_len {
            dfs(y, x, &mut trace, true, &mut dp, &y_len, &x_len, &matrix);
        }
    }

    let mut res = vec![];
    for y in 0..y_len {
        for x in 0..x_len {
            match dp[y][x] {
                Some((true, true)) => {
                    res.push(vec![y as i32, x as i32]);
                }
                _ => {}
            }
        }
    }
    res
}
const DIR: [(usize, usize); 4] = [(0, std::usize::MAX), (0, 1), (std::usize::MAX, 0), (1, 0)];

fn dfs(
    y: usize,
    x: usize,
    trace: &mut Vec<(usize, usize)>,
    reduce: bool, //如果有等值,就不回写了,没出现等值以前的都可以写.
    dp: &mut Vec<Vec<Option<(bool, bool)>>>,
    y_len: &usize,
    x_len: &usize,
    matrix: &Vec<Vec<i32>>,
) -> (bool, bool) {
    match dp[y][x] {
        Some(t) => {
            return t;
        }
        None => {}
    }
    trace.push((y, x));
    let mut to_pacific = false;
    let mut to_atlantic = false;
    if x == 0 || y == 0 {
        to_pacific = true;
    }
    if x == x_len - 1 || y == y_len - 1 {
        to_atlantic = true;
    }
    for &i in DIR.iter() {
        if to_pacific && to_atlantic {
            break;
        }
        let nexty = y.wrapping_add(i.0);
        let nextx = x.wrapping_add(i.1);

        if nexty >= *y_len || nextx >= *x_len {
            continue;
        }
        if trace.contains(&(nexty, nextx)) {
            continue;
        }
        if matrix[nexty][nextx] <= matrix[y][x] {
            let res = if matrix[nexty][nextx] == matrix[y][x] {
                dfs(nexty, nextx, trace, false, dp, y_len, x_len, matrix)
            } else {
                dfs(nexty, nextx, trace, reduce, dp, y_len, x_len, matrix)
            };
            to_pacific = to_pacific || res.0;
            to_atlantic = to_atlantic || res.1;
        }
    }
    trace.pop();
    if reduce {
        dp[y][x] = Some((to_pacific, to_atlantic));
    }
    (to_pacific, to_atlantic)
}
