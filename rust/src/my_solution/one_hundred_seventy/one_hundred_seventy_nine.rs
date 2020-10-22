pub fn largest_number(nums: Vec<i32>) -> String {
    let mut nums_str = nums.iter().map(|r| r.to_string()).collect::<Vec<_>>();
    nums_str.sort_by(|a, b| {
        if a.contains(b) || b.contains(a) {
            return format!("{}{}", b, a).cmp(&format!("{}{}", a, b));
        }
        b.cmp(a)
    });
    if nums_str.len() == 0 || nums_str[0] == "0" {
        return String::from("0");
    }
    nums_str.iter().map(|x| x.to_string()).collect()
}
