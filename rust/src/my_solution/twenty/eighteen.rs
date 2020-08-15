pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut nums = nums;
    let mut result: Vec<Vec<i32>> = vec![];
    let len = nums.len();
    if len < 4 {
        return result;
    }
    nums.sort();

    for i in 0..len - 3 {
        if i != 0 && nums[i] == nums[i - 1] {
            continue;
        }
        for j in i + 1..len - 2 {
            if j != i + 1 && nums[j] == nums[j - 1] {
                continue;
            }
            let mut l = j + 1;
            let mut r = len - 1;
            while l < r {
                if l != j + 1 && nums[l] == nums[l - 1] {
                    l += 1;
                    continue;
                }
                if r != len - 1 && nums[r] == nums[r + 1] {
                    r -= 1;
                    continue;
                }
                let sum = nums[i] + nums[j] + nums[l] + nums[r];
                if sum == target {
                    result.push(vec![nums[i], nums[j], nums[l], nums[r]]);
                    r -= 1;
                    l += 1;
                } else if sum > target {
                    r -= 1;
                } else {
                    l += 1;
                }
            }
        }
    }
    result
}
