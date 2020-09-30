pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    let y_len = grid.len();
    if y_len == 0 {
        return 0;
    }
    let x_len = grid[0].len(); //len应该不会为0，t
    let mut f = vec![0; x_len];
    f[0] = grid[0][0];
    for i in 1..x_len {
        f[i] = f[i - 1] + grid[0][i];
    }
    for y in 1..y_len {
        f[0] = f[0] + grid[y][0];
        for x in 1..x_len {
            f[x] = f[x].min(f[x - 1]) + grid[y][x];
        }
    }
    f[x_len - 1]
}
