pub fn sort_colors(nums: &mut Vec<i32>) {
    let len = nums.len();
    let mut left = 0;
    let mut right = len - 1;
    for i in 0..len {
        if nums[i] == 0 {
            if left == len - 1 {
                return;
            }
            left += 1;
        } else {
            break;
        }
    }
    for i in 0..len {
        if nums[len - 1 - i] == 2 {
            if right == 0 {
                return;
            }
            right -= 1;
        } else {
            break;
        }
    }
    let mut cur = left;
    while cur <= right {
        //        println!(
        //            "{:?},cur = {},left = {}, right = {}",
        //            nums, cur, left, right
        //        );

        match nums[cur] {
            0 => {
                nums.swap(cur, left);
                left += 1;
                cur += 1;
            }
            1 => {
                cur += 1;
            }
            2 => {
                nums.swap(cur, right);
                right -= 1;
            }
            _ => panic!("no way"),
        }
    }
}
