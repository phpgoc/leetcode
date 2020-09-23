use std::borrow::Borrow;

pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let len = intervals.len();
    let mut result = vec![];
    if len == 0 {
        return result;
    }
    let mut intervals = intervals;
    intervals.sort_by(|a, b| a[0].cmp(b[0].borrow()));
    let mut small = intervals[0][0];
    let mut big = intervals[0][1];
    for i in 1..len {
        if intervals[i][1] <= big {
            continue;
        } else if intervals[i][0] <= big {
            big = intervals[i][1];
        } else {
            result.push(vec![small, big]);
            small = intervals[i][0];
            big = intervals[i][1];
        }
    }
    result.push(vec![small, big]);
    result
}
