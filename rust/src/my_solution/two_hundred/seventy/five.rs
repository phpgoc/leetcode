///给定一位研究者论文被引用次数的数组（被引用次数是非负整数），数组已经按照 升序排列 。编写一个方法，计算出研究者的 h 指数。
//
// h 指数的定义: “h 代表“高引用次数”（high citations），一名科研人员的 h 指数是指他（她）的 （N 篇论文中）总共有 h 篇论文分别被引用了至少 h 次。（其余的 N - h 篇论文每篇被引用次数不多于 h 次。）"
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/h-index-ii
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn h_index(citations: Vec<i32>) -> i32 {
    let len = citations.len();
    for i in (0..len).rev() {
        if citations[i] < (len - i) as i32 {
            return (len - i - 1) as i32;
        }
    }
    len as i32
}
