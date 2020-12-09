///给定一个经过编码的字符串，返回它解码后的字符串。
//
// 编码规则为: k[encoded_string]，表示其中方括号内部的 encoded_string 正好重复 k 次。注意 k 保证为正整数。
//
// 你可以认为输入字符串总是有效的；输入字符串中没有额外的空格，且输入的方括号总是符合格式要求的。
//
// 此外，你可以认为原始数据不包含数字，所有的数字只表示重复的次数 k ，例如不会出现像 3a 或 2[4] 的输入。
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/decode-string
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn decode_string(s: String) -> String {
    let chars = s.chars().collect::<Vec<_>>();
    let mut vec = vec![(String::new(), String::new())];
    let mut level = 0;
    for i in chars {
        match i {
            '0'..='9' => {
                vec[level].1.push(i);
            }
            '[' => {
                vec.push((String::new(), String::new()));
                level += 1;
            }
            ']' => {
                let ss = vec[level].0.clone();
                for _ in 0..vec[level - 1].1.parse::<i32>().unwrap() {
                    vec[level - 1].0.push_str(&ss);
                }
                level -= 1;
                vec.pop();
                vec[level].1.clear();
            }
            _ => {
                vec[level].0.push(i);
            }
        }
    }
    vec[0].0.clone()
}
