use std::cmp::Ordering;
///给定一些标记了宽度和高度的信封，宽度和高度以整数对形式 (w, h) 出现。当另一个信封的宽度和高度都比这个信封大的时候，这个信封就可以放进另一个信封里，如同俄罗斯套娃一样。
//
// 请计算最多能有多少个信封能组成一组“俄罗斯套娃”信封（即可以把一个信封放到另一个信封里面）。
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/russian-doll-envelopes
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32 {
    let mut envelopes = envelopes;
    envelopes.sort_by(|a, b| match a[0].cmp(&b[0]) {
        Ordering::Equal => b[1].cmp(&a[1]),
        _a => _a,
    });
    // println!("{:?}", envelopes);

    let mut y = vec![];
    'a: for i in envelopes {
        for j in &mut y {
            if i[1] <= *j {
                *j = i[1];
                continue 'a;
            }
        }
        y.push(i[1]);
    }
    // println!("{:?}", y);
    y.len() as i32
}
