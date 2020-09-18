pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut candidates = candidates;
    candidates.sort();
    let mut result = vec![];
    let mut path = vec![];
    recursive(&candidates, target, 0, &mut result, &mut path);
    result
}

fn recursive(
    candidates: &Vec<i32>,
    target: i32,
    index: usize,
    result: &mut Vec<Vec<i32>>,
    path: &mut Vec<i32>,
) {
    let mut last_pop = 0;
    for i in index..candidates.len() {
        if candidates[i] > target || candidates[i] == last_pop {
            continue;
        } else if candidates[i] < target {
            path.push(candidates[i]);
            recursive(&candidates, target - candidates[i], i + 1, result, path);
            last_pop = path.pop().unwrap();
        } else {
            path.push(candidates[i]);
            result.push(path.clone());
            last_pop = path.pop().unwrap();
        }
    }
}
