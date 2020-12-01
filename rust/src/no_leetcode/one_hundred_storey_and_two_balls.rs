///100层楼扔两个球,找到最低摔碎的层,不确定写的对,就是22-25行注释那里,不一定是对的.
pub fn one_hundred_storey_and_two_balls(mut n: usize) -> Vec<usize> {
    let mut res = vec![];
    let mut map = vec![(0, 0); n + 1];
    map[1] = (1, 1);
    map[2] = (3, 1);
    n_storey_with_two_balls(n, &mut map);
    // println!("{:?}", map);
    while n != 0 {
        res.push(map[n].1);
        n -= map[n].1;
    }
    res
}
fn n_storey_with_two_balls(n: usize, map: &mut Vec<(i32, usize)>) -> (i32, usize) {
    if map[n] != (0, 0) {
        return map[n];
    }
    let mut min_value = std::i32::MAX;
    let mut min_index = std::usize::MAX;
    for i in 1..n {
        let res = ((i + 2) * (i - 1) / 2 ) as i32 //如果第i个碎了,前边每个球的期望
            + i as i32 //如果第i个碎了,依然不能判断i就是最小的,也需要前边每个球都试一次
            + (n - i) as i32 // 后边有n-i个球,每个球都要到后边都需要一次才能到后边
            + n_storey_with_two_balls(n - i, map).0; //后n-i个球的实验总和
        if res < min_value {
            min_value = res;
            min_index = i;
        }
    }
    map[n] = (min_value, min_index);
    return map[n];
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        println!("{:?}", one_hundred_storey_and_two_balls(100));
    }
}
