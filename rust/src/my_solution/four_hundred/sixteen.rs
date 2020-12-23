pub fn can_partition(mut nums: Vec<i32>) -> bool {
    let mut count = 0;
    for &i in &nums {
        count += i;
    }
    if count & 1 == 1 {
        return false;
    }
    let target = count / 2;
    let half = nums.len() / 2;
    nums.sort_by(|a, b| b.cmp(&a));

    let mut map = vec![vec![None; nums.len()]; target as usize];
    can_partition_recursive(0, 0, 0, &nums, &half, &target, &mut map)
}
fn can_partition_recursive(
    position: usize,
    count: usize,
    sum: i32,
    nums: &Vec<i32>,
    half: &usize,
    target: &i32,
    map: &mut Vec<Vec<Option<bool>>>,
) -> bool {
    match map[sum as usize][position] {
        Some(t) => {
            return t;
        }
        _ => {}
    }
    if sum + nums[position] == *target {
        return true;
    }
    if count > *half || position == nums.len() - 1 {
        return false;
    }

    let mut res = can_partition_recursive(position + 1, count, sum, nums, half, target, map);
    if sum + nums[position] < *target {
        res = res
            || can_partition_recursive(
                position + 1,
                count + 1,
                sum + nums[position],
                nums,
                half,
                target,
                map,
            );
    }
    map[sum as usize][position] = Some(res);
    res
}
