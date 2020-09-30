pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
    let mut result = 0;
    let y_len = obstacle_grid.len();
    if y_len == 0 {
        return result;
    }
    let x_len = obstacle_grid[0].len(); //len应该不会为0，target不会为负

    let mut f = vec![0; x_len];
    if obstacle_grid[0][0] == 1 {
        return result;
    } else {
        f[0] = 1;
    }
    for x in 1..x_len {
        if obstacle_grid[0][x] == 0 && f[x - 1] == 1 {
            f[x] = 1;
        }
    }
    for y in 1..y_len {
        for x in 0..x_len {
            if obstacle_grid[y][x] == 0 {
                if x == 0 {
                    f[x] = f[x];
                } else {
                    f[x] += f[x - 1];
                }
            } else {
                f[x] = 0;
            }
        }
    }
    f[x_len - 1]
}
