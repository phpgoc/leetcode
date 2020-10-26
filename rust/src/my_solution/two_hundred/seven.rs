pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    let mut can_do = vec![vec![]; num_courses as usize];
    for i in &prerequisites {
        can_do[i[0] as usize].push(i[1] as usize);
    }
    loop {
        let mut do_nonthing = true;
        'a: for i in 0..num_courses as usize {
            if !can_do[i].is_empty() {
                for j in &can_do[i] {
                    if !can_do[*j].is_empty() {
                        continue 'a;
                    }
                }
                can_do[i].clear();
                do_nonthing = false;
            }
        }
        if do_nonthing {
            break;
        }
    }
    for i in can_do {
        if !i.is_empty() {
            return false;
        }
    }
    true
}
