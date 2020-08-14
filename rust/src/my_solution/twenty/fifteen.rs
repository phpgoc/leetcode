pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec![];
    if nums.len() <= 2 {
        return result;
    }
    let mut nums = nums;
    nums.sort();
    let mut unique_nums = vec![];
    let mut cur = nums[0];
    let mut count = 1;
    unique_nums.push(cur);
    for i in 1..nums.len() {
        if nums[i] != cur {
            cur = nums[i];
            count = 1;
            unique_nums.push(cur);
            continue;
        }
        count += 1;
        if cur == 0 {
            if count == 3 {
                result.push(vec![0, 0, 0]);
            }
        } else {
            if count == 2 {
                unique_nums.push(cur);
            }
        }
    }
    if unique_nums.len() <= 2 {
        return result;
    }
    for i in 0..unique_nums.len() - 2 {
        if i != 0 && unique_nums[i - 1] == unique_nums[i] {
            continue;
        }
        let mut l = i + 1;
        let mut r = unique_nums.len() - 1;
        let target = -unique_nums[i];
        while r > l {
            let sum = unique_nums[r] + unique_nums[l];
            if sum > target {
                r -= 1;
            } else if sum < target {
                l += 1;
            } else {
                result.push(vec![unique_nums[i], unique_nums[l], unique_nums[r]]);
                if unique_nums[l] == unique_nums[l + 1] {
                    l += 2;
                } else {
                    l += 1;
                }
                if unique_nums[r] == unique_nums[r - 1] {
                    r -= 2;
                } else {
                    r -= 1;
                }
            }
        }
    }
    result
}
