pub mod eighteen;
pub mod eighty;
pub mod fifty;
pub mod forty;
pub mod four;
pub mod nine;
pub mod nineteen;
pub mod ninty;
pub mod seven;
pub mod seventy;
pub mod six;
pub mod sixteen;
pub mod sixty;
pub mod thirty;
pub mod three;
pub mod twenty;
pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut dp = vec![1; len];
    for i in 0..len {
        for j in 0..i {
            if nums[j] < nums[i] {
                dp[i] = dp[i].max(dp[j] + 1);
            }
        }
    }
    let mut res = 0;
    for i in dp {
        res = res.max(i);
    }
    res
}
mod use_greed {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut dp = vec![];
        let mut res = 0;
        'a: for i in nums {
            if dp.is_empty() || dp[dp.len() - 1] < i {
                dp.push(i);
                res += 1;
            } else {
                let mut l = 0;
                let mut r = dp.len() - 1;
                while l < r {
                    let mid = l + (r - l) / 2;
                    if dp[mid] > i {
                        r = mid;
                    } else if dp[mid] < i {
                        l = mid + 1;
                    } else {
                        continue 'a;
                    }
                }
                dp[l] = i;
            }
        }
        res
    }
}
#[cfg(test)]
mod tests;
