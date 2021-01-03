use std::cmp::Ordering;
///给定一个区间的集合，找到需要移除区间的最小数量，使剩余区间互不重叠。
pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
    let mut intervals = intervals;
    intervals.sort_by(|a, b| match a[0].cmp(&b[0]) {
        Ordering::Equal => a[1].cmp(&b[1]),
        _a => _a,
    });
    let mut max = i32::MIN;
    let mut count = 0;
    for i in intervals {
        if i[0] >= max {
            max = i[1];
        } else {
            count += 1;
            max = max.min(i[1]);
        }
    }
    count
}
