use std::collections::HashSet;

pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
    let mut i1 = 0;
    let mut i2 = 0;
    let mut i3 = 0;
    let mut posible_i1_i2 = HashSet::new();
    let s1_v = s1.chars().collect::<Vec<_>>();
    let s2_v = s2.chars().collect::<Vec<_>>();
    let s3_v = s3.chars().collect::<Vec<_>>();
    let s1_len = s1_v.len();
    let s2_len = s2_v.len();
    let s3_len = s3_v.len();
    if s1_len + s2_len != s3_len {
        return false;
    }
    while i3 < s3_len {
        if posible_i1_i2.is_empty() {
            if i1 == s1_len {
                if s2_v[i2] == s3_v[i3] {
                    i2 += 1;
                    i3 += 1;
                    continue;
                } else {
                    return false;
                }
            }
            if i2 == s2_len {
                if s1_v[i1] == s3_v[i3] {
                    i1 += 1;
                    i3 += 1;
                    continue;
                } else {
                    return false;
                }
            }
            if s1_v[i1] == s3_v[i3] && s2_v[i2] == s3_v[i3] {
                posible_i1_i2.insert((i1, i2 + 1));
                posible_i1_i2.insert((i1 + 1, i2));
                i3 += 1;
            } else if s1_v[i1] == s3_v[i3] {
                i1 += 1;
                i3 += 1;
            } else if s2_v[i2] == s3_v[i3] {
                i2 += 1;
                i3 += 1;
            } else {
                return false;
            }
        } else {
            let last_posible = posible_i1_i2;
            posible_i1_i2 = HashSet::new();
            for (i, j) in last_posible {
                if i == s1_len {
                    if s2_v[j] == s3_v[i3] {
                        posible_i1_i2.insert((i, j + 1));
                    }
                    continue;
                }
                if j == s2_len {
                    if s1_v[i] == s3_v[i3] {
                        posible_i1_i2.insert((i + 1, j));
                    }
                    continue;
                }
                if s1_v[i] == s3_v[i3] && s2_v[j] == s3_v[i3] {
                    posible_i1_i2.insert((i, j + 1));
                    posible_i1_i2.insert((i + 1, j));
                } else if s1_v[i] == s3_v[i3] {
                    posible_i1_i2.insert((i + 1, j));
                } else if s2_v[j] == s3_v[i3] {
                    posible_i1_i2.insert((i, j + 1));
                }
            }
            if posible_i1_i2.len() == 0 {
                return false;
            } else if posible_i1_i2.len() == 1 {
                let only_1 = posible_i1_i2.iter().collect::<Vec<(_)>>()[0];
                i1 = only_1.0;
                i2 = only_1.1;
                posible_i1_i2.clear();
            }
            i3 += 1;
        }
    }
    true
}
