///还记得童话《卖火柴的小女孩》吗？现在，你知道小女孩有多少根火柴，请找出一种能使用所有火柴拼成一个正方形的方法。不能折断火柴，可以把火柴连接起来，并且每根火柴都要用到。
///
///输入为小女孩拥有火柴的数目，每根火柴用其长度表示。输出即为是否能用所有的火柴拼成正方形。
pub fn makesquare(mut nums: Vec<i32>) -> bool {
    let mut max = 0;
    let mut sum = 0;
    for &i in nums.iter() {
        sum += i;
        max = max.max(i);
    }
    if sum == 0 || sum & 3 != 0 {
        return false;
    }
    let target = sum / 4;
    if max > target {
        return false;
    }
    let mut visited = vec![false; nums.len()];
    nums.sort_by(|a, b| b.cmp(a));
    for _ in 0..3 {
        let index = visited.iter().position(|r| !*r).unwrap();
        if !dfs(index, &mut visited, &nums, target, 0) {
            return false;
        }
    }
    true
}
fn dfs(position: usize, visited: &mut Vec<bool>, nums: &Vec<i32>, target: i32, sum: i32) -> bool {
    if nums[position] + sum == target {
        visited[position] = true;
        return true;
    } else if nums[position] + sum > target {
        return false;
    }
    for k in position + 1..nums.len() {
        if !visited[k] && dfs(k, visited, nums, target, sum + nums[position]) {
            visited[position] = true;
            return true;
        }
    }
    false
}
