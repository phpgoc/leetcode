///找出所有相加之和为 n 的 k 个数的组合。组合中只允许含有 1 - 9 的正整数，并且每种组合中不存在重复的数字。
//
// 说明：
//
// 所有数字都是正整数。
// 解集不能包含重复的组合。
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/combination-sum-iii
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
    let mut res = vec![];
    let mut cur = vec![];
    if k < 0 || k > 9 || n < k * (k + 1) / 2 || n > 45 - (10 - k) * (9 - k) / 2 {
        return res;
    }
    dfs(&mut res, &mut cur, k, 0, &n, 1);
    // println!("{:?}", res);
    res
}
fn dfs(res: &mut Vec<Vec<i32>>, cur: &mut Vec<i32>, k: i32, count: i32, n: &i32, index: i32) {
    let down = *n - count - k * (k - 1) / 2;
    // println!("down = {}", down);
    if index * k > down {
        return;
    } else if index * k == down {
        for i in 0..k {
            cur.push(index + i);
        }
        res.push(cur.clone());
        for _ in 0..k {
            cur.pop();
        }
        return;
    }
    let up = 45 - (10 - k) * (9 - k) / 2 - *n + count;
    // println!("up = {}", up);

    if up < 0 {
        return;
    } else if up == 0 {
        let mut i = 9;
        for _ in 0..k {
            cur.push(i);
            i -= 1;
        }
        res.push(cur.clone());
        for _ in 0..k {
            cur.pop();
        }
        return;
    }
    if k == 1 {
        cur.push(*n - count);
        res.push(cur.clone());
        cur.pop();
        return;
    }

    for i in index..=9 {
        cur.push(i);
        dfs(res, cur, k - 1, count + i, &n, i + 1);
        cur.pop();
    }
}
