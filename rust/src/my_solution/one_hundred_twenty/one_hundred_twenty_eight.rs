use std::borrow::Borrow;
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let mut set = nums.iter().collect::<HashSet<_>>();
    let mut res = 0;
    for i in &nums {
        if !set.contains((i - 1).borrow()) {
            let mut cur = *i;
            let mut cur_res = 1;
            while set.contains((cur + 1).borrow()) {
                cur_res += 1;
                cur += 1;
            }
            res = res.max(cur_res);
        }
    }
    res
}
