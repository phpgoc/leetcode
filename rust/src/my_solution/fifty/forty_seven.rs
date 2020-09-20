use std::collections::HashSet;

pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    nums.sort();

    let mut answer = vec![];
    let mut for_select = HashSet::new();
    for i in 0..nums.len() {
        for_select.insert(i);
    }
    let mut path = vec![];
    recursive(&nums, &mut for_select, &mut path, &mut answer);
    answer
}

fn recursive(
    nums: &Vec<i32>,
    for_select: &mut HashSet<usize>,
    path: &mut Vec<i32>,
    answer: &mut Vec<Vec<i32>>,
) {
    if for_select.is_empty() {
        answer.push(path.clone());
    }
    for i in for_select.clone() {
        if i != 0 && for_select.contains(&(i - 1)) && nums[i] == nums[i - 1] {
            continue;
        }
        path.push(nums[i]);
        for_select.remove(&i);
        recursive(nums, for_select, path, answer);
        path.pop();
        for_select.insert(i);
    }
}
