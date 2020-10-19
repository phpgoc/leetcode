pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
    if numerator == 0 {
        return String::from("0");
    }
    if numerator < 0 {
        return if denominator < 0 {
            i64_fraction_to_decimal(0 - numerator as i64, 0 - denominator as i64)
        } else {
            format!(
                "-{}",
                i64_fraction_to_decimal(0 - numerator as i64, denominator as i64)
            )
        };
    }
    if denominator < 0 {
        return format!(
            "-{}",
            i64_fraction_to_decimal(numerator as i64, 0 - denominator as i64)
        );
    }
    i64_fraction_to_decimal(numerator as i64, denominator as i64)
}
fn i64_fraction_to_decimal(numerator: i64, denominator: i64) -> String {
    let mut res = String::new();
    let mut left = numerator;
    let mut quetient = (left / denominator).to_string();
    left = left % denominator;
    res.push_str(&quetient);
    if left != 0 {
        res.push('.');
    } else {
        return res;
    }
    let mut vec = vec![];
    loop {
        let mut str = String::new();
        loop {
            left *= 10;
            quetient = (left / denominator).to_string();
            if quetient == "0" {
                str.push('0');
            } else {
                str.push_str(&quetient);
                left %= denominator;
                break;
            }
        }
        if vec.contains(&(left, str.clone())) {
            for (i, vec_str) in vec {
                if i == left && vec_str == str {
                    res.push('(');
                }
                res.push_str(&*vec_str);
            }
            res.push(')');
            break;
        } else {
            vec.push((left, str));
        }
        if left == 0 {
            for i in vec {
                res.push_str(&*i.1);
            }
            break;
        }
    }

    res
}
