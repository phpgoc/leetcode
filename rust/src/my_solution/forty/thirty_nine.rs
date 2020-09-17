pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut result = vec![];
    let mut path = vec![];
    recursive(&candidates, target, 0, &mut result, &mut path);
    result
}

fn recursive(
    candidates: &Vec<i32>,
    target: i32,
    last_number: i32,
    result: &mut Vec<Vec<i32>>,
    path: &mut Vec<i32>,
) {
    for &i in candidates {
        if i > target || i < last_number {
            continue;
        } else if i < target {
            path.push(i);
            recursive(&candidates, target - i, i, result, path);
            path.pop();
        } else {
            path.push(i);
            result.push(path.clone());
            path.pop();
        }
    }
}
