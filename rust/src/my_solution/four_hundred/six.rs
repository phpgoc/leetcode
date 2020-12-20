use std::cmp::Ordering;
///假设有打乱顺序的一群人站成一个队列，数组 people 表示队列中一些人的属性（不一定按顺序）。每个 people[i] = [hi, ki] 表示第 i 个人的身高为 hi ，前面 正好 有 ki 个身高大于或等于 hi 的人。
//
// 请你重新构造并返回输入数组 people 所表示的队列。返回的队列应该格式化为数组 queue ，其中 queue[j] = [hj, kj] 是队列中第 j 个人的属性（queue[0] 是排在队列前面的人）。
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/queue-reconstruction-by-height
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut people = people;
    let len = people.len();
    people.sort_by(|a, b| match a[0].cmp(&b[0]) {
        Ordering::Equal => b[1].cmp(&a[1]),
        _a => _a,
    });
    let mut res = vec![vec![]; len];
    for i in people {
        let mut count = 0;
        for j in 0..len {
            if res[j].is_empty() {
                if count == i[1] {
                    res[j] = i;
                    break;
                }
                count += 1;
            }
        }
    }
    res
}
