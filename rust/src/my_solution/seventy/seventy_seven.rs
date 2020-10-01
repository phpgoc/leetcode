pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    let mut path = vec![];
    let mut result = vec![];
    dfs(&n, 1, k, &mut path, &mut result);
    result
}

fn dfs(n: &i32, from: i32, left: i32, path: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
    if left == 0 {
        result.push((*path).clone());
        return;
    }
    for i in from..=n + 1 - left {
        path.push(i);
        dfs(n, i + 1, left - 1, path, result);
        path.pop();
    }
}
