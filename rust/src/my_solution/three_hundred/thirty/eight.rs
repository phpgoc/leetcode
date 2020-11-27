///给定一个非负整数 num。对于 0 ≤ i ≤ num 范围中的每个数字 i ，计算其二进制数中的 1 的数目并将它们作为数组返回。
pub fn count_bits(num: i32) -> Vec<i32> {
    let mut res = vec![0];
    let target_len = num as usize + 1;
    while res.len() < target_len {
        res.extend(res.clone().iter().map(|r| r + 1));
    }
    res.drain(target_len..);
    res
}
