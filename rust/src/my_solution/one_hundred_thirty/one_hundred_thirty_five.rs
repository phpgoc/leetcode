pub fn candy(ratings: Vec<i32>) -> i32 {
    let len = ratings.len();
    let mut res = len as i32;
    let mut up = 1;
    let mut down = 1;
    let mut status = Status::None;
    for i in 1..len {
        if ratings[i] > ratings[i - 1] {
            match status {
                Status::Down | Status::Flat => {
                    res += down * (down - 1) / 2;
                    res -= up.min(down) - 1;
                    up = 2;
                }
                Status::Up | Status::None => {
                    up += 1;
                }
            }
            status = Status::Up;
        } else if ratings[i] == ratings[i - 1] {
            match status {
                Status::Down => {
                    res += down * (down - 1) / 2;
                    res -= up.min(down) - 1;
                }
                Status::Up => {
                    res += up * (up - 1) / 2;
                }
                Status::Flat | Status::None => {}
            }
            up = 1;
            down = 1;
            status = Status::Flat;
        } else {
            match status {
                Status::Up | Status::Flat => {
                    res += up * (up - 1) / 2;
                    down = 2;
                }
                Status::Down | Status::None => {
                    down += 1;
                }
            }
            status = Status::Down;
        }
        //        println!(
        //            "status = {:?},res = {},up = {},down = {}",
        //            status, res, up, down
        //        );
    }
    match status {
        Status::Up => {
            res += up * (up - 1) / 2;
        }
        Status::Down => {
            res += down * (down - 1) / 2;
            if up != 0 {
                res -= up.min(down) - 1;
            }
        }
        _ => {}
    }
    res
}

#[derive(Debug)]
enum Status {
    Up,
    Down,
    Flat,
    None,
}
