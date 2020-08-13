use std::convert::TryInto;
use std::iter::FromIterator;

pub fn convert(s: String, num_rows: i32) -> String {
    if num_rows == 1 {
        return s;
    }
    let mut origin_vec = vec![vec![]; num_rows.try_into().unwrap()];
    let mut c = 0;
    let mut direction = 1;
    let chars: Vec<char> = s.chars().collect();
    for i in chars {
        origin_vec[c].push(i);
        if (direction == 1) {
            c += 1;
        } else {
            c -= 1;
        }
        if c == 0 || c == (num_rows - 1) as usize {
            direction = direction * -1;
        }
    }
    let one_vec = origin_vec.concat();
    String::from_iter(one_vec)
}
