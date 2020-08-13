pub fn length_of_longest_substring(s: String) -> i32 {
    if s.is_empty() {
        return 0;
    }
    let mut max: i32 = 1;
    let mut l = 0;
    let mut s1;
    for (i, v) in s.chars().enumerate() {
        if i < 1 {
            continue;
        }
        s1 = &s[l..i];
        if let Some(p) = s1.find(v) {
            l = l + p + 1;
        } else {
            max = std::cmp::max(max, (i - l + 1) as i32);
        }
    }
    max
}
