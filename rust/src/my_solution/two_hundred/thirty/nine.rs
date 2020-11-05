use std::collections::VecDeque;

pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut deque = VecDeque::new();
    let k = k as usize;
    if k == 1{
        return nums;
    }
    let mut res = vec![];

    for i in 0..k{
        while !deque.is_empty() &&nums[deque[0]] < nums[i]{
            deque.pop_front();
        }
        while !deque.is_empty() && nums[deque[deque.len()-1]] < nums[i]{
            deque.pop_back();
        }
        deque.push_back(i);
    }
    res.push(nums[deque[0]]);
    for i in k..nums.len(){
        if deque[0] == i-k{
            deque.pop_front();
        }
        while !deque.is_empty() &&nums[deque[0]] < nums[i]{
            deque.pop_front();
        }
        while !deque.is_empty() && nums[deque[deque.len()-1]] < nums[i]{
            deque.pop_back();
        }
        deque.push_back(i);
        // println!("{:?}",deque);
        res.push(nums[deque[0]]);
    }
    res
}