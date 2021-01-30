///两个整数之间的汉明距离指的是这两个数字对应二进制位不同的位置的数目。
//
// 给出两个整数 x 和 y，计算它们之间的汉明距离。
pub fn hamming_distance(x: i32, y: i32) -> i32 {
    let mut bit_or = x ^ y;
    let mut res = 0;
    while bit_or != 0 {
        res += bit_or & 1;
        bit_or >>= 1;
    }
    res
}
