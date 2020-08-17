use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;
lazy_static! {
    static ref CACHE: Mutex<HashMap<i32, Vec<String>>> = {
        let mut map = HashMap::new();
        map.insert(0, vec![String::from("")]);
        map.insert(1, vec![String::from("()")]);
        Mutex::new(map)
    };
}
pub fn generate_parenthesis(n: i32) -> Vec<String> {
    generate_parenthesis_recursion(n)
}

fn generate_parenthesis_recursion(n: i32) -> Vec<String> {
    let mut map = CACHE.lock().unwrap();

    if let Some(t) = map.get(&n) {
        return t.clone();
    }
    drop(map);
    let mut result = vec![];
    for i in 0..n {
        for l in generate_parenthesis_recursion(i) {
            for r in generate_parenthesis_recursion(n - i - 1) {
                let str = format!("({}){}", l, r);
                result.push(str);
                //                println!("{}", str);
            }
        }
    }
    let mut map = CACHE.lock().unwrap();
    map.insert(n, result.clone());
    result
}
