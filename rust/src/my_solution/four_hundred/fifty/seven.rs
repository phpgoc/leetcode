use std::collections::HashSet;

///给定一个含有正整数和负整数的环形数组 nums。 如果某个索引中的数 k 为正数，则向前移动 k 个索引。相反，如果是负数 (-k)，则向后移动 k 个索引。因为数组是环形的，所以可以假设最后一个元素的下一个元素是第一个元素，而第一个元素的前一个元素是最后一个元素。
//
// 确定 nums 中是否存在循环（或周期）。循环必须在相同的索引处开始和结束并且循环长度 > 1。此外，一个循环中的所有运动都必须沿着同一方向进行。换句话说，一个循环中不能同时包括向前的运动和向后的运动。
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/circular-array-loop
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn circular_array_loop(nums: Vec<i32>) -> bool {
    let len = nums.len();
    let mut visited = vec![false; len];
    let mut trace = HashSet::new();
    let mut dir = true; //true right right left
    'a: for i in 0..nums.len() {
        if visited[i] {
            continue;
        }
        trace.clear();
        if nums[i] > 0 {
            dir = true;
        } else {
            dir = false;
        }
        let mut cur = i;
        while !visited[cur] && ((nums[cur] < 0) ^ dir) {
            trace.insert(cur);
            visited[cur] = true;
            let next = (cur + (len as i32 + nums[cur]) as usize) % len;
            if cur == next {
                continue 'a;
            }
            if trace.contains(&next) {
                return true;
            }
            cur = next;
        }
    }

    false
}
