///给定一位研究者论文被引用次数的数组（被引用次数是非负整数）。编写一个方法，计算出研究者的 h 指数。
//
// h 指数的定义：h 代表“高引用次数”（high citations），一名科研人员的 h 指数是指他（她）的 （N 篇论文中）总共有 h 篇论文分别被引用了至少 h 次。（其余的 N - h 篇论文每篇被引用次数 不超过 h 次。）
//
// 例如：某人的 h 指数是 20，这表示他已发表的论文中，每篇被引用了至少 20 次的论文总共有 20 篇。
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/h-index
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn h_index(citations: Vec<i32>) -> i32 {
    let mut citations = citations;
    citations.sort();
    let len = citations.len();
    for i in (0..len).rev() {
        if citations[i] < (len - i) as i32 {
            return (len - i - 1) as i32;
        }
    }
    len as i32
}
