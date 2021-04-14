use std::collections::HashSet;
pub fn construct_rectangle(area: i32) -> Vec<i32> {
    if PRIME.contains(&area) {
        return vec![area, 1];
    }
    let mut factors = vec![];
    let mut area_for_readuce = area;
    let mut i = 2;
    while !PRIME.contains(&area_for_readuce) {
        if area_for_readuce % i == 0 {
            factors.push(i);
            area_for_readuce /= i;
        } else {
            i += 1;
        }
    }

    if area_for_readuce != 1 {
        factors.push(area_for_readuce);
    }
    let sqrt = (area as f32).sqrt() as i32;
    let mut width = 1;
    find_max_width(&mut width, &factors, 0, factors.len() - 1, &sqrt, 1);
    vec![area / width, width]
}
lazy_static::lazy_static! {
    static ref PRIME: HashSet<i32> = {
        let to = 10000 ;
        let mut set = HashSet::new();
        for i in 1..to {
            set.insert(i);
        }
        let sqrt = (to as f64).sqrt() as i32;
        for i in 2..=sqrt {
            if !set.contains(&i) {
                continue;
            }
            for j in i..to {
                if j > to/i {
                    break;
                }
                set.remove(&(i*j));
            }
        }
        // println!("{:?}",set);
        set
    };
}
fn find_max_width(
    width: &mut i32,
    factors: &Vec<i32>,
    from: i32, //前一个位置不考虑的值，这次也不会再考虑了，3,3,3，前边的3退回的时候，后边也跳过
    cur: usize,
    sqrt: &i32,
    res: i32,
) -> bool {
    if factors[cur] == from {
        if cur == 0 {
            return false;
        } else {
            return find_max_width(width, factors, from, cur - 1, sqrt, res);
        }
    }
    if res * factors[cur] == *sqrt {
        *width = *sqrt;
        return true;
    } else if res * factors[cur] > *sqrt {
        if cur == 0 {
            return false;
        } else {
            return find_max_width(width, factors, from, cur - 1, sqrt, res);
        }
    } else {
        *width = *width.max(&mut (res * factors[cur]));
        if cur == 0 {
            return false;
        }
        find_max_width(width, factors, from, cur - 1, sqrt, res * factors[cur])
            || find_max_width(width, factors, from, cur - 1, sqrt, res)
    }
}
