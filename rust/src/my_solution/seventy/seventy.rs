use std::collections::HashMap;

pub fn climb_stairs(n: i32) -> i32 {
    let mut map = HashMap::new();
    map.insert(0, 0);
    map.insert(1, 1);
    map.insert(2, 2);
    recursive(n, &mut map)
}

fn recursive(n: i32, map: &mut HashMap<i32, i32>) -> i32 {
    return match map.get(&n) {
        Some(&t) => t,
        None => {
            let result = recursive(n - 1, map) + recursive(n - 2, map);
            map.insert(n, result);
            result
        }
    };
}
