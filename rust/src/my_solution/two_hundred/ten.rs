///现在你总共有 n 门课需要选，记为 0 到 n-1。
//
// 在选修某些课程之前需要一些先修课程。 例如，想要学习课程 0 ，你需要先完成课程 1 ，我们用一个匹配来表示他们: [0,1]
//
// 给定课程总量以及它们的先决条件，返回你为了学完所有课程所安排的学习顺序。
//
// 可能会有多个正确的顺序，你只要返回一种就可以了。如果不可能完成所有课程，返回一个空数组。
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/course-schedule-ii
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
    let mut can_do = vec![vec![]; num_courses as usize];
    for i in &prerequisites {
        can_do[i[0] as usize].push(i[1] as usize);
    }
    let mut res = vec![];
    for i in 0..num_courses as usize {
        if can_do[i].is_empty() {
            res.push(i as i32);
        }
    }
    loop {
        let mut do_nonthing = true;
        'a: for i in 0..num_courses as usize {
            if !can_do[i].is_empty() {
                for j in &can_do[i] {
                    if !can_do[*j].is_empty() {
                        continue 'a;
                    }
                }
                can_do[i].clear();
                res.push(i as i32);
                do_nonthing = false;
            }
        }
        if do_nonthing {
            break;
        }
    }

    if res.len() == num_courses as usize {
        return res;
    } else {
        return vec![];
    }
}
