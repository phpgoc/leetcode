pub fn str_str(haystack: String, needle: String) -> i32 {
    let needle_len = needle.len();
    let haystack_len = haystack.len();
    let haystack_chars: Vec<char> = haystack.chars().collect();
    if needle_len == 0 {
        return 0;
    }
    if haystack_len < needle_len {
        return -1;
    }
    let mut cur = 0;
    loop {
        if cur + needle_len >= haystack_len {
            if needle.eq(&String::from(
                &haystack[haystack_len - needle_len..haystack_len],
            )) {
                return (haystack_len - needle_len) as i32;
            } else {
                return -1;
            }
        }
        if needle.eq(&String::from(&haystack[cur..cur + needle_len])) {
            return cur as i32;
        }

        match needle.rfind(haystack_chars[cur + needle_len]) {
            None => {
                cur += needle_len;
            }
            Some(T) => {
                cur = cur + needle_len - T;
            }
        }
    }
}
