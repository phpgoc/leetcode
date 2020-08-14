use std::cmp::min;

pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
    let mut nums = nums;
    nums.sort();

    let mut result = 1e4 as i32;
    for i in 0..nums.len() - 2 {
        if i != 0 && nums[i - 1] == nums[i] {
            continue;
        }
        let mut l = i + 1;
        let mut r = nums.len() - 1;
        let mut sum = nums[i] + nums[l] + nums[r];
        let mut last_sum = sum;
        let mut minus = sum < target;
        while l < r {
            sum = nums[i] + nums[l] + nums[r];
            if sum == target {
                return target;
            } else if sum > target {
                if minus {
                    result = update(result, last_sum, target);
                    minus = !minus;
                }
                r -= 1;
            } else {
                if !minus {
                    result = update(result, last_sum, target);
                    minus = !minus;
                }
                l += 1;
            }
            last_sum = sum;
        }
        result = update(result, last_sum, target);
    }
    result
}

fn update(result: i32, new_v: i32, target: i32) -> i32 {
    if (result - target).abs() > (new_v - target).abs() {
        new_v
    } else {
        result
    }
}
