///给定一个无重复元素的有序整数数组 nums 。
//
// 返回 恰好覆盖数组中所有数字 的 最小有序 区间范围列表。也就是说，nums 的每个元素都恰好被某个区间范围所覆盖，并且不存在属于某个范围但不属于 nums 的数字 x 。
//
// 列表中的每个区间范围 [a,b] 应该按如下格式输出：
//
// "a->b" ，如果 a != b
// "a" ，如果 a == b
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/summary-ranges
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    let mut res = vec![];
    if nums.len() == 0 {
        return res;
    }
    let mut min = nums[0];
    let mut max = nums[0];
    for &i in nums[1..].iter() {
        if i != max + 1 {
            if min == max {
                res.push(min.to_string());
            } else {
                res.push(format!("{}->{}", min, max));
            }
            min = i;
            max = i;
        } else {
            max = i;
        }
    }
    if min == max {
        res.push(min.to_string());
    } else {
        res.push(format!("{}->{}", min, max));
    }
    res
}
