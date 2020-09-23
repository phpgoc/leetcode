pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    let mut intervals = intervals;
    let len = intervals.len();
    if len == 0 {
        return vec![new_interval];
    }

    if intervals[len - 1][1] < new_interval[0] {
        intervals.insert(len, new_interval);
        return intervals;
    }
    if intervals[0][0] > new_interval[1] {
        intervals.insert(0, new_interval);
        return intervals;
    }
    let mut for_delete = vec![];
    let mut for_insert = 0;
    let mut small = 0;
    let mut big = 0;
    for i in 0..len {
        if new_interval[1] < intervals[i][0] {
            if for_delete.is_empty() {
                for_insert = i;
                small = new_interval[0];
                big = new_interval[1];
            }
            break;
        } else if new_interval[0] > intervals[i][1] {
            continue;
        } else {
            if for_delete.is_empty() {
                for_insert = i;
                small = intervals[i][0].min(new_interval[0]);
            }
            for_delete.push(i);

            big = intervals[i][1].max(new_interval[1]);
            //            println!("small = {}, big = {}", small, big);
        }
    }

    // 空下边也没问题
    for_delete.reverse();
    for i in for_delete {
        intervals.remove(i);
    }

    intervals.insert(for_insert, vec![small, big]);

    //    println!("{:?}", intervals);
    intervals
}
