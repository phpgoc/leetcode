use std::cmp::{max, min};

pub fn max_area(height: Vec<i32>) -> i32 {
    let mut l = 0;
    let mut r = height.len() - 1;
    let mut max_volume = 0;
    while l != r {
        max_volume = max(max_volume, min(height[l], height[r]) * (r - l) as i32);
        if height[r] < height[l] {
            r -= 1;
        } else {
            l += 1;
        }
    }
    max_volume
}
