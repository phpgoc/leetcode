pub fn simplify_path(path: String) -> String {
    let str_vec: Vec<_> = path.split('/').collect();
    let mut ans_vec = vec![];
    for i in str_vec {
        match i {
            "" | "." => {
                //do nothing
            }
            ".." => {
                ans_vec.pop(); //没有也没事
            }

            _ => ans_vec.push(i),
        }
    }
    //    println!("{:?}", ans_vec);

    let mut ans = String::new();
    for i in ans_vec {
        ans.push('/');
        ans.push_str(i);
    }
    if ans.is_empty() {
        ans.push('/');
    }
    ans
}
