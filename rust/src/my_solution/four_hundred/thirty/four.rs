///统计字符串中的单词个数，这里的单词指的是连续的不是空格的字符。
pub fn count_segments(s: String) -> i32 {
    let mut count = 0;
    let chars = s.chars().collect::<Vec<_>>();
    let mut blank = true;
    for i in chars {
        if i == ' ' {
            if !blank {
                count += 1;
            }
            blank = true;
        } else {
            blank = false;
        }
    }
    if !blank {
        count += 1;
    }
    count
}
