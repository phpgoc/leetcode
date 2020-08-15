pub fn is_valid(s: String) -> bool {
    let mut chars: Vec<char> = s.chars().collect();
    let mut stack = vec![];
    for char in chars {
        if char == '(' || char == '[' || char == '{' {
            stack.push(char);
        } else {
            if char == ')' {
                let pop = stack.pop();
                if pop.is_none() || pop.unwrap() != '(' {
                    return false;
                }
            } else if char == ']' {
                let pop = stack.pop();
                if pop.is_none() || pop.unwrap() != '[' {
                    return false;
                }
            } else if char == '}' {
                let pop = stack.pop();
                if pop.is_none() || pop.unwrap() != '{' {
                    return false;
                }
            }
        }
    }
    stack.is_empty()
}
