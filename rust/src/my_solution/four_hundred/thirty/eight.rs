///给定一个字符串 s 和一个非空字符串 p，找到 s 中所有是 p 的字母异位词的子串，返回这些子串的起始索引。
//
// 字符串只包含小写英文字母，并且字符串 s 和 p 的长度都不超过 20100。
//
//
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/find-all-anagrams-in-a-string
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
    let p_len = p.len();
    let s_len = s.len();
    let p_chars = p.chars().map(|r| r as usize - 97).collect::<Vec<_>>();
    let mut target = vec![0; 26];
    let mut hash = vec![0; 26];
    for i in p_chars.iter() {
        target[*i] += 1;
    }
    let s_chars = s.chars().map(|r| r as usize - 97).collect::<Vec<_>>();
    let mut res = vec![];
    let mut from = 0;
    if !padding(&mut from, &mut hash, &s_chars, &target, &p_len) {
        return res;
    }
    while from + p_len <= s_len {
        if hash == target {
            res.push(from as i32);
        }
        if from + p_len < s_len {
            if target[s_chars[from + p_len]] == 0 {
                from += p_len + 1;
                if !padding(&mut from, &mut hash, &s_chars, &target, &p_len) {
                    return res;
                }
                continue;
            }

            hash[s_chars[from]] -= 1;
            hash[s_chars[from + p_len]] += 1;
        }

        from += 1;
    }
    res
}
fn padding(
    from: &mut usize,
    hash: &mut Vec<i32>,
    s_chars: &Vec<usize>,
    target: &Vec<i32>,
    p_len: &usize,
) -> bool {
    'a: loop {
        if *from + p_len > s_chars.len() {
            return false;
        }
        *hash = vec![0; 26];
        for i in *from..*from + p_len {
            if target[s_chars[i]] == 0 {
                *from = i + 1;
                continue 'a;
            }
            hash[s_chars[i]] += 1;
        }
        return true;
    }
}
