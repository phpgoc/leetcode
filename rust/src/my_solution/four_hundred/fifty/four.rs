use std::collections::BTreeMap;

///给定四个包含整数的数组列表 A , B , C , D ,计算有多少个元组 (i, j, k, l) ，使得 A[i] + B[j] + C[k] + D[l] = 0。
//
// 为了使问题简单化，所有的 A, B, C, D 具有相同的长度 N，且 0 ≤ N ≤ 500 。所有整数的范围在 -228 到 228 - 1 之间，最终结果不会超过 231 - 1 。
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/4sum-ii
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn four_sum_count(a: Vec<i32>, b: Vec<i32>, mut c: Vec<i32>, mut d: Vec<i32>) -> i32 {
    c.sort();
    d.sort();
    let mut a_b_sum_map = BTreeMap::new();
    for &a_i in a.iter() {
        for &b_i in b.iter() {
            let sum = a_i + b_i;
            let count = a_b_sum_map.entry(sum).or_insert(0);
            *count += 1;
        }
    }
    let mut res = 0;
    for &c_i in c.iter() {
        for &d_i in d.iter() {
            let sum = -c_i - d_i;
            if let Some(&t) = a_b_sum_map.get(&sum) {
                res += t;
            } else if a_b_sum_map.range(..sum).next_back().is_none() {
                break;
            }
        }
    }
    res
}
