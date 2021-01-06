///给定一组字符，使用原地算法将其压缩。
//
// 压缩后的长度必须始终小于或等于原数组长度。
//
// 数组的每个元素应该是长度为1 的字符（不是 int 整数类型）。
pub fn compress(chars: &mut Vec<char>) -> i32 {
    let mut cur = ' ';
    let mut c = 0;
    for i in (0..chars.len()).rev() {
        if chars[i] == cur {
            c += 1;
        } else {
            if c > 1 {
                chars.drain(i + 2..=i + c);
                for char in c.to_string().chars().rev() {
                    chars.insert(i + 2, char);
                }
            }
            c = 1;
            cur = chars[i];
        }
    }
    if c > 1 {
        chars.drain(1..=c - 1);
        for char in c.to_string().chars().rev() {
            chars.insert(1, char);
        }
    }
    chars.len() as i32
}
