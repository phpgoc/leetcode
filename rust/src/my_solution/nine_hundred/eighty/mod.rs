use std::collections::HashMap;

///在二维网格 grid 上，有 4 种类型的方格：
//
// 1 表示起始方格。且只有一个起始方格。
// 2 表示结束方格，且只有一个结束方格。
// 0 表示我们可以走过的空方格。
// -1 表示我们无法跨越的障碍。
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/unique-paths-iii
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
const DIR: [(usize, usize); 4] = [(0, std::usize::MAX), (0, 1), (std::usize::MAX, 0), (1, 0)];

pub fn unique_paths_iii(grid: Vec<Vec<i32>>) -> i32 {
    let y_len = grid.len();
    let x_len = grid[0].len();
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut map = HashMap::new();
    let mut count = 0;
    for y in 0..y_len {
        for x in 0..x_len {
            match grid[y][x] {
                1 => {
                    start = (y, x);
                    count += 1;
                }
                2 => {
                    end = (y, x);
                    count += 1;
                }
                0 => {
                    count += 1;
                }
                _ => {}
            }
        }
    }
    dfs(start, 0, count, &mut map, &end, &y_len, &x_len, &grid)
}

fn dfs(
    posotion: (usize, usize),
    mut visited: usize,
    mut left: i32,
    map: &mut HashMap<(usize, (usize, usize)), i32>,
    end: &(usize, usize),
    y_len: &usize,
    x_len: &usize,
    grid: &Vec<Vec<i32>>,
) -> i32 {
    visited += (1 << (posotion.0 * x_len + posotion.1));

    if let Some(&t) = map.get(&(visited, posotion)) {
        return t;
    }
    left -= 1;
    if posotion == *end {
        //不需要记忆
        if left == 0 {
            return 1;
        }
        return 0;
    }

    let mut res = 0;
    for &i in DIR.iter() {
        let nexty = posotion.0.wrapping_add(i.0);
        let nextx = posotion.1.wrapping_add(i.1);

        if nextx < *x_len && nexty < *y_len && grid[nexty][nextx] != -1 {
            if (visited & (1 << (nexty * x_len + nextx))) == 0 {
                res += dfs((nexty, nextx), visited, left, map, end, y_len, x_len, grid);
            }
        }
    }
    visited -= 1 << (posotion.0 * x_len + posotion.1);
    map.insert((visited, posotion), res);
    res
}
#[cfg(test)]
mod tests;
