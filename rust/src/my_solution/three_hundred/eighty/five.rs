#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}
///给定一个用字符串表示的整数的嵌套列表，实现一个解析它的语法分析器。
//
// 列表中的每个元素只可能是整数或整数嵌套列表
//
// 提示：你可以假定这些字符串都是格式良好的：
//
// 字符串非空
// 字符串不包含空格
// 字符串只包含数字0-9、[、-、,、]
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/mini-parser
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn deserialize(mut s: String) -> NestedInteger {
    if s.starts_with('[') {
        s.remove(0);
        s.pop();
        let mut num = 0;
        let mut start = 0;
        let chars = s.chars().collect::<Vec<char>>();
        let mut vec_res = vec![];
        for (k, &v) in chars.iter().enumerate() {
            match v {
                ',' => {
                    if num == 0 {
                        vec_res.push(deserialize(String::from(&s[start..k])));
                        start = k + 1;
                    }
                }
                '[' => {
                    num += 1;
                }
                ']' => {
                    num -= 1;
                }
                _ => {}
            }
        }
        vec_res.push(deserialize(String::from(&s[start..])));
        NestedInteger::List(vec_res)
    } else {
        NestedInteger::Int(s.parse::<i32>().unwrap())
    }
}
