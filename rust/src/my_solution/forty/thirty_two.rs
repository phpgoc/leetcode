pub fn longest_valid_parentheses(s: String) -> i32 {
    let mut count = 0;
    let chars: Vec<char> = s.chars().collect();
    let mut left_index = vec![];
    let len = chars.len();
    let mut result = 0;

    for i in 0..len {
        if chars[i] == '(' {
            if left_index.get(count).is_none() {
                left_index.push(i);
            }
            count += 1;
        } else {
            if count == 0 {
                left_index.clear();
                continue;
            } else {
                count -= 1;
                if (count < left_index.len() - 1) {
                    left_index.pop();
                }
                result = result.max(i - left_index[count] + 1);
            }
        }
        //        println!("vector {:?}", left_index);
        //        println!("i = {}", i);
        //        println!("result = {}", result);
    }
    result as i32
}
