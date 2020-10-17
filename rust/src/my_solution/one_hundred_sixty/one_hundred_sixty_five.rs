pub fn compare_version(version1: String, version2: String) -> i32 {
    let v1 = version1.split('.').collect::<Vec<_>>();
    let v2 = version2.split('.').collect::<Vec<_>>();
    for i in 0..5 {
        let s1 = if i < v1.len() {
            v1[i].parse::<i32>().unwrap()
        } else {
            0
        };
        let s2 = if i < v2.len() {
            v2[i].parse::<i32>().unwrap()
        } else {
            0
        };

        if s1 < s2 {
            return -1;
        } else if s1 > s2 {
            return 1;
        }
    }
    0
}
