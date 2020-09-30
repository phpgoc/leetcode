pub fn is_number(s: String) -> bool {
    let mut s = s;
    s = s.trim().parse().unwrap();
    let chars: Vec<_> = s.chars().collect();
    let len = chars.len();
    //    println!("{:?}", chars);
    let mut doted = false;
    let mut has_e = false;
    let mut started_number = false;
    for (k, v) in chars.iter().enumerate() {
        match v {
            '+' | '-' => {
                if !started_number {
                    if k == chars.len() - 1 {
                        return false;
                    }
                    match chars[k + 1] {
                        '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' | '.' => {
                            continue;
                        }
                        _ => {
                            return false;
                        }
                    }
                    started_number = true;
                } else {
                    return false;
                }
            }
            'e' => {
                if !has_e && started_number && k != len - 1 {
                    match chars[k + 1] {
                        '+' | '-' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0'
                        | '.' => {
                            has_e = true;
                            started_number = false;
                        }
                        _ => {
                            return false;
                        }
                    }
                } else {
                    return false;
                }
            }
            '.' => {
                if doted || has_e {
                    return false;
                } else {
                    doted = true;
                    if !started_number {
                        if k == len - 1 {
                            return false;
                        }

                        match chars[k + 1] {
                            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                                started_number = true;
                            }
                            _ => {
                                return false;
                            }
                        }
                    }
                }
            }
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                started_number = true;
            }
            _ => {
                return false;
            }
        }
    }
    started_number
}
