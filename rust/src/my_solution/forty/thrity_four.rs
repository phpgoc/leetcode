pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result = vec![-1, -1];
    let len = nums.len();
    if len == 0 {
        return result;
    }
    let mut l = 0;
    let mut r = len - 1;
    if nums[l] > target || nums[r] < target {
        return result;
    }
    let mut one_target_value = -1;
    while l <= r {
        let mid = l + (r - l) / 2;
        if nums[mid] == target {
            one_target_value = mid as i32;
            break;
        } else if nums[mid] > target {
            r = mid - 1;
            continue;
        } else {
            l = mid + 1;
            continue;
        }
    }
    if one_target_value == -1 {
        return result;
    }
    let one_target_value = one_target_value as usize;
    println!("one---{}", one_target_value);
    if one_target_value == 0 || nums[one_target_value - 1] != target {
        result[0] = one_target_value as i32;
        result[1] = find_min_or_max(&nums, one_target_value, len - 1, target, false);
    } else if one_target_value == len - 1 || nums[one_target_value + 1] != target {
        result[0] = find_min_or_max(&nums, 0, one_target_value, target, true);
        result[1] = one_target_value as i32
    } else {
        result[0] = find_min_or_max(&nums, 0, one_target_value, target, true);
        result[1] = find_min_or_max(&nums, one_target_value, len - 1, target, false);
    }
    result
}

fn find_min_or_max(vec: &Vec<i32>, l: usize, r: usize, target: i32, min: bool) -> i32 {
    let mut l = l;
    let mut r = r;
    let mut result = if min { r } else { l };
    while l <= r {
        let mid = l + (r - l) / 2;
        if vec[mid] == target {
            result = mid;
            if min {
                if mid == 0 {
                    break;
                }
                r = mid - 1;
            } else {
                l = mid + 1;
            }
            result = mid;
        } else {
            if min {
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }
    }
    result as i32
}
