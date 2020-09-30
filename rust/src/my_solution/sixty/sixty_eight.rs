pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
    let max_width = max_width as usize;
    let mut cur_from = 0;
    let mut len = 0;
    let mut count = 0;
    let mut result = vec![];
    for (k, v) in words.iter().enumerate() {
        if len + v.len() > max_width {
            add_to_vec(
                cur_from,
                k - 1,
                &count,
                &len,
                &max_width,
                &words,
                &mut result,
            );
            cur_from = k;
            len = v.len() + 1;
            count = 1;
        } else {
            len += v.len() + 1;
            count += 1;
        }
        if k == words.len() - 1 {
            add_to_vec(cur_from, k, &count, &len, &max_width, &words, &mut result);
        }
    }
    result
}

fn add_to_vec(
    from: usize,
    to: usize,
    count: &usize,
    len: &usize,
    max_width: &usize,
    words: &Vec<String>,
    result: &mut Vec<String>,
) {
    if *count == 1 {
        let mut str = format!("{:width$}", words[from], width = *max_width);
        result.push(str);
        return;
    }
    let mut add_to_vec = String::new();
    if to == words.len() - 1 && *len != max_width + 1 {
        for i in 0..*count {
            add_to_vec.push_str(words[from + i].as_ref());
            add_to_vec.push(' ');
        }
        add_to_vec = format!("{:width$}", add_to_vec, width = *max_width);
        result.push(add_to_vec);
        return;
    }
    let mut blank_vec = vec![1; count - 1];
    //    println!(
    //        "max_width =  {}, len = {}, from = {}, to = {}",
    //        *max_width, *len, from, to
    //    );
    for i in 0..*max_width + 1 - *len {
        blank_vec[i % (*count - 1)] += 1;
    }
    //    println!("{:?}", blank_vec);

    for i in 0..*count - 1 {
        add_to_vec.push_str(words[from + i].as_ref());
        for _ in 0..blank_vec[i] {
            add_to_vec.push(' ');
        }
    }
    add_to_vec.push_str(words[to].as_ref());
    result.push(add_to_vec);
}
