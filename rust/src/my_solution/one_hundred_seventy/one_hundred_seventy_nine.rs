pub fn largest_number(nums: Vec<i32>) -> String {
    let mut nums_str = nums.iter().map(|r| r.to_string()).collect::<Vec<_>>();
    nums_str.sort_by(|a, b| {
        let mut a1 = a.clone();
        a1.push_str(b);
        let mut b1 = b.clone();
        b1.push_str(a);
        b1.cmp(&a1)
    });
    // println!("{:?}", nums_str);
    if nums_str.len() == 0 || nums_str[0] == "0" {
        return String::from("0");
    }
    let mut res = String::new();

    for i in nums_str.iter() {
        res.push_str(i);
    }
    res
}
