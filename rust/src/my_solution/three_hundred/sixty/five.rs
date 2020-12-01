use std::collections::HashSet;
///有两个容量分别为 x升 和 y升 的水壶以及无限多的水。请判断能否通过使用这两个水壶，从而可以得到恰好 z升 的水？
//
// 如果可以，最后请用以上水壶中的一或两个来盛放取得的 z升 水。
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/water-and-jug-problem
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn can_measure_water(x: i32, y: i32, z: i32) -> bool {
    let mut set = HashSet::new();
    let mut stack = vec![];
    stack.push((0, 0));
    while let Some(t) = stack.pop() {
        let mut next = vec![];
        next.push((0, t.1));
        next.push((t.0, t.1));
        next.push((x, t.1));
        next.push((t.0, y));
        next.push((t.0 - t.0.min(y - t.1), t.1 + t.0.min(y - t.1)));
        next.push((t.0 + t.1.min(x - t.0), t.1 - t.1.min(x - t.0)));
        for i in next {
            if set.contains(&i) {
                continue;
            }
            if i.0 == z || i.1 == z || i.0 + i.1 == z {
                return true;
            }
            stack.push(i);
            set.insert(i);
        }
    }
    false
}
