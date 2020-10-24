/// #  岛屿数量
/// 给你一个由 '1'（陆地）和 '0'（水）组成的的二维网格，请你计算网格中岛屿的数量。
///
/// 岛屿总是被水包围，并且每座岛屿只能由水平方向和/或竖直方向上相邻的陆地连接形成。
///
/// 此外，你可以假设该网格的四条边均被水包围。
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/number-of-islands
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    let y = grid.len();
    let x = grid.first().unwrap().len();
    let mut has_considered = vec![vec![false; x]; y];
    let mut res = 0;
    for y_i in 0..y {
        for x_i in 0..x {
            if has_considered[y_i][x_i] == false {
                dfs(x_i, y_i, &x, &y, &grid, &mut res, &mut has_considered, true);
            }
        }
    }
    res
}
fn dfs(
    x: usize,
    y: usize,
    x_len: &usize,
    y_len: &usize,
    grid: &Vec<Vec<char>>,
    res: &mut i32,
    has_considered: &mut Vec<Vec<bool>>,
    add: bool,
) {
    if grid[y][x] == '0' || has_considered[y][x] {
        return;
    }
    if add {
        *res += 1;
    }
    has_considered[y][x] = true;
    if x != 0 {
        dfs(x - 1, y, x_len, y_len, grid, res, has_considered, false);
    }
    if y != 0 {
        dfs(x, y - 1, x_len, y_len, grid, res, has_considered, false);
    }
    if x != x_len - 1 {
        dfs(x + 1, y, x_len, y_len, grid, res, has_considered, false);
    }
    if y != y_len - 1 {
        dfs(x, y + 1, x_len, y_len, grid, res, has_considered, false);
    }
}
