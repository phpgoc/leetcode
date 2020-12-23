///给定一个非空数组，返回此数组中第三大的数。如果不存在，则返回数组中最大的数。要求算法时间复杂度必须是O(n)。
pub fn third_max(nums: Vec<i32>) -> i32 {
    let mut vec = vec![];
    for i in nums {
        if vec.contains(&i) {
            continue;
        }
        if vec.len() < 3 {
            vec.push(i);
            vec.sort_by(|a, b| b.cmp(a));
            continue;
        }
        if vec[2] > i {
            continue;
        }
        let p = vec.iter().position(|&r| r < i).unwrap();
        vec.insert(p, i);
        vec.drain(3..);
    }

    if vec.len() < 3 {
        vec[0]
    } else {
        vec[2]
    }
}
