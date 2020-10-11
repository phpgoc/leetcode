pub fn get_row(row_index: i32) -> Vec<i32> {
    let mut res = vec![];
    let row_index = row_index as i64;
    let mut index = row_index;
    let mut c: i64 = 1;
    for i in 1..=row_index + 1 {
        res.push(c as i32);
        c = c * index / i;

        index -= 1;
    }
    res
}
