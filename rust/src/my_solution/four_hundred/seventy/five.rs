use std::collections::BTreeSet;

///冬季已经来临。 你的任务是设计一个有固定加热半径的供暖器向所有房屋供暖。
//
// 在加热器的加热半径范围内的每个房屋都可以获得供暖。
//
// 现在，给出位于一条水平线上的房屋 houses 和供暖器 heaters 的位置，请你找出并返回可以覆盖所有房屋的最小加热半径。
//
// 说明：所有供暖器都遵循你的半径标准，加热的半径也一样。
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/heaters
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn find_radius(houses: Vec<i32>, heaters: Vec<i32>) -> i32 {
    let mut max = 0;
    let tree = heaters.into_iter().collect::<BTreeSet<_>>();
    for i in houses {
        max = max.max(
            (i - *tree.range(..i).next_back().unwrap_or(&-1000000000_i32))
                .min((tree.range(i..).next().unwrap_or(&i32::MAX) - i)),
        );
    }
    max
}
