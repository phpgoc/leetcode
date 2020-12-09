///UTF-8 中的一个字符可能的长度为 1 到 4 字节，遵循以下的规则：
//
// 对于 1 字节的字符，字节的第一位设为0，后面7位为这个符号的unicode码。
// 对于 n 字节的字符 (n > 1)，第一个字节的前 n 位都设为1，第 n+1 位设为0，后面字节的前两位一律设为10。剩下的没有提及的二进制位，全部为这个符号的unicode码。
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/utf-8-validation
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn valid_utf8(data: Vec<i32>) -> bool {
    let mut next_bit = 0;
    for i in data {
        match i {
            0..=127 => {
                if next_bit != 0 {
                    return false;
                }
            }
            240..=247 => {
                if next_bit != 0 {
                    return false;
                }
                next_bit = 3;
            }
            224..=239 => {
                if next_bit != 0 {
                    return false;
                }
                next_bit = 2;
            }
            192..=223 => {
                if next_bit != 0 {
                    return false;
                }
                next_bit = 1;
            }
            128..=191 => {
                if next_bit == 0 {
                    return false;
                }
                next_bit -= 1;
            }
            _ => {
                return false;
            }
        }
    }
    next_bit == 0
}
