pub fn max_product(nums: Vec<i32>) -> i32 {
    let mut cur_res = 1;
    let mut minus_cur_res = 1;
    let mut minus_after_cur_res = 1;
    let mut res = std::i32::MIN;
    let mut first_minus = false;

    for i in nums {
        if i == 0 {
            cur_res = 1;
            minus_cur_res = 1;
            minus_after_cur_res = 1;
            first_minus = false;
            res = res.max(0);
        } else {
            let mut first_in = false;
            if !first_minus && i < 0 {
                first_minus = true;
                first_in = true;
            }
            if first_minus {
                minus_cur_res *= i;
                res = res.max(minus_cur_res);
                if !first_in {
                    minus_after_cur_res *= i;
                    res = res.max(minus_after_cur_res);
                }
            }
            cur_res *= i;
            res = res.max(cur_res);
        }
    }
    res
}
