///给定长度分别为 m 和 n 的两个数组，其元素由 0-9 构成，表示两个自然数各位上的数字。现在从这两个数组中选出 k (k <= m + n) 个数字拼接成一个新的数，要求从同一个数组中取出的数字保持其在原数组中的相对顺序。
//
// 求满足该条件的最大数。结果返回一个表示该最大数的长度为 k 的数组。
//
// 说明: 请尽可能地优化你算法的时间和空间复杂度。
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/create-maximum-number
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn max_number(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i32> {
    let len1 = nums1.len();
    let len2 = nums2.len();
    let mut left = k as usize;
    let mut res = vec![];
    let mut from = vec![(0, 0)];
    let mut next_from = vec![];
    let mut cur_max = -1;
    while left != 0 {
        for &i in &from {
            for i1 in i.0..len1.min(1 + len1 + len2 - left - i.1) {
                if nums1[i1] > cur_max {
                    next_from.clear();
                    cur_max = nums1[i1];
                    next_from.push((i1 + 1, i.1));
                }

                if nums1[i1] == 9 {
                    break;
                }
            }
            let mut done = false;
            for i2 in i.1..len2.min(1 + len1 + len2 - left - i.0) {
                if !done && nums2[i2] == cur_max {
                    next_from.push((i.0, i2 + 1));
                    done = true;
                }
                if nums2[i2] > cur_max {
                    next_from.clear();
                    cur_max = nums2[i2];
                    done = true;
                    next_from.push((i.0, i2 + 1));
                }
                if nums2[i2] == 9 {
                    break;
                }
            }
        }
        res.push(cur_max);
        from = next_from.drain(0..).collect();
        cur_max = -1;
        left -= 1;
    }
    res
}
