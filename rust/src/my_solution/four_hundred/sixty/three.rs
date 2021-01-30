///给定一个 row x col 的二维网格地图 grid ，其中：grid[i][j] = 1 表示陆地， grid[i][j] = 0 表示水域。
//
// 网格中的格子 水平和垂直 方向相连（对角线方向不相连）。整个网格被水完全包围，但其中恰好有一个岛屿（或者说，一个或多个表示陆地的格子相连组成的岛屿）。
//
// 岛屿中没有“湖”（“湖” 指水域在岛屿内部且不和岛屿周围的水相连）。格子是边长为 1 的正方形。网格为长方形，且宽度和高度均不超过 100 。计算这个岛屿的周长。
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/island-perimeter
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
    let mut res = 0;
    let (y_len, x_len) = match grid.first() {
        Some(t) => (grid.len(), t.len()),
        _ => {
            return 0;
        }
    };
    for y in 0..y_len {
        for x in 0..x_len {
            if grid[y][x] == 1 {
                res += 4;
                if y != y_len - 1 {
                    res -= grid[y + 1][x] * 2;
                }
                if x != x_len - 1 {
                    res -= grid[y][x + 1] * 2;
                }
            }
        }
    }
    res
}
