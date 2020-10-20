use std::collections::HashMap;

pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
    let (x_len, y_len) = match dungeon.first() {
        Some(t) => ((*t).len(), dungeon.len()),
        _ => return 0,
    };
    let mut map = HashMap::new();
    let last = dungeon[y_len - 1][x_len - 1];
    if last >= 0 {
        map.insert((y_len - 1, x_len - 1), 0);
    } else {
        map.insert((y_len - 1, x_len - 1), -last);
    }
    dfs(0, 0, &dungeon, &x_len, &y_len, &mut map) + 1
}
fn dfs(
    y: usize,
    x: usize,
    dungeon: &Vec<Vec<i32>>,
    x_len: &usize,
    y_len: &usize,
    map: &mut HashMap<(usize, usize), i32>,
) -> i32 {
    match map.get(&(y, x)) {
        Some(t) => {
            return *t;
        }
        _ => {}
    };

    let cur = dungeon[y][x];
    let mut min_need = std::i32::MAX;
    if y != y_len - 1 {
        min_need = min_need.min(dfs(y + 1, x, dungeon, x_len, y_len, map));
    }
    if x != x_len - 1 {
        min_need = min_need.min(dfs(y, x + 1, dungeon, x_len, y_len, map));
    }
    let res = 0.max(min_need - cur);
    map.insert((y, x), res);
    res
}
