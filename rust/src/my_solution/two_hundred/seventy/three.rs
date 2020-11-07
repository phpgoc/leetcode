///将非负整数 num 转换为其对应的英文表示。
pub fn number_to_words(num: i32) -> String {
    let mut num = num;
    if num == 0 {
        return String::from("Zero");
    }
    let mut res = String::new();
    if num >= 1_000_000_000 {
        res.push_str(&three_bit(num / 1_000_000_000));
        res.push_str("Billion ");
        num %= 1_000_000_000;
    }
    if num >= 1_000_000 {
        res.push_str(&three_bit(num / 1_000_000));
        res.push_str("Million ");
        num %= 1_000_000;
    }
    if num >= 1e3 as i32 {
        res.push_str(&three_bit(num / 1_000));
        res.push_str("Thousand ");
        num %= 1_000;
    }
    if num > 0 {
        res.push_str(&three_bit(num));
    }
    res[0..res.len() - 1].to_string()
}
fn three_bit(mut num: i32) -> String {
    let mut res = String::new();
    match num / 100 {
        9 => {
            res.push_str("Nine Hundred ");
        }
        8 => {
            res.push_str("Eight Hundred ");
        }
        7 => {
            res.push_str("Seven Hundred ");
        }
        6 => {
            res.push_str("Six Hundred ");
        }
        5 => {
            res.push_str("Five Hundred ");
        }
        4 => {
            res.push_str("Four Hundred ");
        }
        3 => {
            res.push_str("Three Hundred ");
        }
        2 => {
            res.push_str("Two Hundred ");
        }
        1 => {
            res.push_str("One Hundred ");
        }
        _ => {}
    }
    num %= 100;
    match num / 10 {
        9 => {
            res.push_str("Ninety ");
        }
        8 => {
            res.push_str("Eighty ");
        }
        7 => {
            res.push_str("Seventy ");
        }
        6 => {
            res.push_str("Sixty ");
        }
        5 => {
            res.push_str("Fifty ");
        }
        4 => {
            res.push_str("Forty ");
        }
        3 => {
            res.push_str("Thirty ");
        }
        2 => {
            res.push_str("Twenty ");
        }
        1 => {
            match num % 10 {
                9 => {
                    res.push_str("Nineteen ");
                }
                8 => {
                    res.push_str("Eighteen ");
                }
                7 => {
                    res.push_str("Seventeen ");
                }
                6 => {
                    res.push_str("Sixteen ");
                }
                5 => {
                    res.push_str("Fifteen ");
                }
                4 => {
                    res.push_str("Fourteen ");
                }
                3 => {
                    res.push_str("Thirteen ");
                }
                2 => {
                    res.push_str("Twelve ");
                }
                1 => {
                    res.push_str("Eleven ");
                }
                _ => {
                    res.push_str("Ten ");
                }
            }
            return res;
        }
        _ => {}
    }
    match num % 10 {
        9 => {
            res.push_str("Nine ");
        }
        8 => {
            res.push_str("Eight ");
        }
        7 => {
            res.push_str("Seven ");
        }
        6 => {
            res.push_str("Six ");
        }
        5 => {
            res.push_str("Five ");
        }
        4 => {
            res.push_str("Four ");
        }
        3 => {
            res.push_str("Three ");
        }
        2 => {
            res.push_str("Two ");
        }
        1 => {
            res.push_str("One ");
        }
        _ => {}
    }
    res
}
