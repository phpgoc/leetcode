use rand::rngs::ThreadRng;
use rand::Rng;

///给定圆的半径和圆心的 x、y 坐标，写一个在圆中产生均匀随机点的函数 randPoint 。
//
// 说明:
//
// 输入值和输出值都将是浮点数。
// 圆的半径和圆心的 x、y 坐标将作为参数传递给类的构造函数。
// 圆周上的点也认为是在圆中。
// randPoint 返回一个包含随机点的x坐标和y坐标的大小为2的数组。
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/generate-random-point-in-a-circle
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub struct Solution {
    radius: f64,
    x_center: f64,
    y_center: f64,
    rng: ThreadRng,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    pub fn new(radius: f64, x_center: f64, y_center: f64) -> Self {
        Self {
            radius,
            x_center,
            y_center,
            rng: rand::thread_rng(),
        }
    }

    pub fn rand_point(&mut self) -> Vec<f64> {
        loop {
            let for_y = (self.rng.gen::<f64>() - 0.5) / 0.5;
            let for_x = (self.rng.gen::<f64>() - 0.5) / 0.5;
            if for_x * for_x + for_y * for_y > 1.0 {
                continue;
            }
            return vec![
                self.x_center + for_x * self.radius,
                self.y_center + for_y * self.radius,
            ];
        }
    }
}
