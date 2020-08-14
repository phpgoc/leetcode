pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.len() == 0 {
        return String::new();
    }
    let mut min_string = &strs[0];
    let mut max_string = &strs[0];
    for i in 0..strs.len() {
        if (strs[i].lt(min_string)) {
            min_string = &strs[i];
        }
        if (strs[i].gt(max_string)) {
            max_string = &strs[i];
        }
    }
    if min_string.eq(max_string) {
        return max_string.clone();
    }
    let mut min_len = min_string.len();
    let mut max_len = max_string.len();

    min_len = min_len.min(max_len);
    for i in 0..min_len {
        if !min_string.starts_with(&max_string[0..=i]) {
            return String::from(&max_string[0..i]);
        }
    }
    String::from(&max_string[0..min_len])
}
