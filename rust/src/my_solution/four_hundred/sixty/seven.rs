///把字符串 s 看作是“abcdefghijklmnopqrstuvwxyz”的无限环绕字符串，所以 s 看起来是这样的："...zabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcd....".
//
// 现在我们有了另一个字符串 p 。你需要的是找出 s 中有多少个唯一的 p 的非空子串，尤其是当你的输入是字符串 p ，你需要输出字符串 s 中 p 的不同的非空子串的数目。
//
// 注意: p 仅由小写的英文字母组成，p 的大小可能超过 10000。
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/unique-substrings-in-wraparound-string
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn find_substring_in_wrapround_string(p: String) -> i32 {
    if p.len() == 0 {
        return 0;
    }
    let mut counter = vec![0; 128];
    let chars = p.chars().map(|r| r as usize).collect::<Vec<_>>();
    let mut from = chars[0];
    let mut count = 1;
    for i in 1..chars.len() {
        if chars[i] == chars[i - 1] + 1 || (chars[i] == 97 && chars[i - 1] == 122) {
            count += 1;
        } else {
            for j in (1.max(count - 25)..=count).rev() {
                if counter[from] > j {
                    break;
                } else {
                    counter[from] = j;
                }

                from = (from - 96) % 26 + 97;
            }
            from = chars[i];
            count = 1;
        }
    }
    for j in (1.max(count - 25)..=count).rev() {
        counter[from] = counter[from].max(j);
        from = (from - 96) % 26 + 97;
    }

    counter.iter().sum()
}
