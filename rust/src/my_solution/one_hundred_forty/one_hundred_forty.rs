use std::collections::{HashMap, HashSet};

pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
    let mut word_dict = word_dict;
    let len = s.len();
    word_dict.sort();
    let mut map = HashMap::new();
    let mut max_len = 0;
    for v in word_dict.iter() {
        max_len = max_len.max(v.len());
        map.insert((*v).clone(), vec![]);
    }
    let mut contained_set: HashSet<String> = HashSet::new();
    for i in 1..word_dict.len() {
        if !contained_set.is_empty() {
            let set_clone = contained_set.clone();
            for j in set_clone {
                if word_dict[i].starts_with(&j) {
                    map.get_mut(&j).unwrap().push(word_dict[i].clone());
                } else {
                    contained_set.remove(&j);
                }
            }
        }
        if word_dict[i].contains(&word_dict[i - 1]) {
            map.get_mut(&word_dict[i - 1])
                .unwrap()
                .push(word_dict[i].clone());
            contained_set.insert(word_dict[i - 1].clone());
        }
    }
    let mut res_map = vec![None; len];
    let mut str_vec = vec![];
    let mut res = vec![];
    //    println!("{:?}", map);
    word_break_recursive(
        0,
        &max_len,
        &len,
        &map,
        &s,
        &mut res_map,
        &mut str_vec,
        &mut res,
    );
    //    println!("{:?}", res);
    res
}

fn word_break_recursive(
    from: usize,
    max_len: &usize,
    len: &usize,
    map: &HashMap<String, Vec<String>>,
    s: &str,
    res_map: &mut Vec<Option<bool>>,
    str_vec: &mut Vec<String>,
    result: &mut Vec<String>,
) -> bool {
    if from == *len {
        result.push(str_vec.join(" "));
        return true;
    }
    match res_map[from] {
        Some(false) => {
            return false;
        }
        _ => {}
    }
    let do_time = (*len - from).min(*max_len);
    //    println!("from = {},do_time = {}", from, do_time);
    for i in 0..do_time {
        //        println!("{}", &s[from..=from + i]);
        match map.get(&s[from..=from + i]) {
            Some(t) => {
                //                println!("t = {:?}", t);

                if t.is_empty() {
                    str_vec.push((&s[from..=from + i].to_string()).parse().unwrap());
                    let res = word_break_recursive(
                        from + i + 1,
                        max_len,
                        len,
                        map,
                        s,
                        res_map,
                        str_vec,
                        result,
                    );
                    str_vec.pop();
                    res_map[from] = Some(res);
                    return res;
                } else {
                    let mut res = false;

                    for j in t.iter() {
                        if (&s[from..]).starts_with(j) {
                            str_vec.push(j.clone());
                            res = word_break_recursive(
                                from + j.len(),
                                max_len,
                                len,
                                map,
                                s,
                                res_map,
                                str_vec,
                                result,
                            ) || res;
                            str_vec.pop();
                        }
                    }
                    str_vec.push((&s[from..=from + i].to_string()).parse().unwrap());
                    res = word_break_recursive(
                        from + i + 1,
                        max_len,
                        len,
                        map,
                        s,
                        res_map,
                        str_vec,
                        result,
                    ) || res;
                    str_vec.pop();
                    res_map[from] = Some(res);

                    return res;
                }
            }
            None => {}
        }
    }
    res_map[from] = Some(false);
    false
}
