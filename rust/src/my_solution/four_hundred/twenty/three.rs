use std::collections::{BTreeMap, HashMap};
///给定一个非空字符串，其中包含字母顺序打乱的英文单词表示的数字0-9。按升序输出原始的数字。
//
// 注意:
//
// 输入只包含小写英文字母。
// 输入保证合法并可以转换为原始的数字，这意味着像 "abc" 或 "zerone" 的输入是不允许的。
// 输入字符串的长度小于 50,000。
// 示例 1:
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/reconstruct-original-digits-from-english
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn original_digits(s: String) -> String {
    let mut res = String::new();

    let chars = s.chars().collect::<Vec<_>>();
    let mut words_count = HashMap::new();
    for i in [
        'g', 'v', 'r', 't', 'i', 'o', 'f', 'w', 'h', 'n', 'z', 's', 'e', 'x', 'u',
    ]
    .iter()
    {
        words_count.insert(i, 0);
    }

    for i in chars {
        let count = words_count.get_mut(&i).unwrap();
        *count += 1;
    }
    let mut res_hash = BTreeMap::new();

    res_hash.insert('0', *words_count.get(&'z').unwrap());
    res_hash.insert('2', *words_count.get(&'w').unwrap());
    res_hash.insert('4', *words_count.get(&'u').unwrap());
    res_hash.insert('6', *words_count.get(&'x').unwrap());
    res_hash.insert('8', *words_count.get(&'g').unwrap());
    res_hash.insert(
        '3',
        *words_count.get(&'h').unwrap() - *words_count.get(&'g').unwrap(),
    );
    res_hash.insert(
        '5',
        *words_count.get(&'f').unwrap() - *words_count.get(&'u').unwrap(),
    );
    res_hash.insert(
        '7',
        *words_count.get(&'s').unwrap() - *words_count.get(&'x').unwrap(),
    );
    res_hash.insert(
        '1',
        *words_count.get(&'o').unwrap()
            - *words_count.get(&'z').unwrap()
            - *words_count.get(&'u').unwrap()
            - *words_count.get(&'w').unwrap(),
    );
    res_hash.insert(
        '9',
        *words_count.get(&'i').unwrap()
            - *words_count.get(&'x').unwrap()
            - *words_count.get(&'g').unwrap()
            + *words_count.get(&'u').unwrap()
            - *words_count.get(&'f').unwrap(),
    );
    for (&k, &v) in res_hash.iter() {
        for _ in 0..v {
            res.push(k);
        }
    }

    res
}
