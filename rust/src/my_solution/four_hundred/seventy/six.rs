///给定一个正整数，输出它的补数。补数是对该数的二进制表示取反。
pub fn find_complement(num: i32) -> i32 {
    let mut all_one = i32::MAX;
    while all_one >> 1 >= num {
        all_one >>= 1;
    }
    all_one - num
}
