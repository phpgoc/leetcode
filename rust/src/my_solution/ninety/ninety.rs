pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    nums.sort();
    let mut result = vec![vec![]];
    let mut cur = std::i32::MAX;
    let mut last_len = 0;
    let mut cur_len = result.len();
    for i in nums {
        if i == cur {
            cur_len = result.len();
            for j in last_len..cur_len {
                let mut cur_v = result[j].clone();
                cur_v.push(i);
                result.push(cur_v);
            }
            last_len = cur_len;
        } else {
            last_len = result.len();

            for j in 0..last_len {
                let mut cur_v = result[j].clone();
                cur_v.push(i);
                result.push(cur_v);
            }
        }
        cur = i;
    }
    //    println!("{:?}", result);

    result
}
