pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack = vec![];
    for i in tokens {
        if ("+-*/".find(i.as_str()).is_none()) {
            stack.push(i.parse::<i32>().unwrap());
        } else {
            let b = match stack.pop() {
                Some(t) => t,
                None => return -1,
            };
            let a = match stack.pop() {
                Some(t) => t,
                None => return -1,
            };
            if "*".find(i.as_str()).is_some() {
                stack.push(a * b);
            } else if "-".find(i.as_str()).is_some() {
                stack.push(a - b);
            } else if "+".find(i.as_str()).is_some() {
                stack.push(a + b);
            } else if "/".find(i.as_str()).is_some() {
                stack.push(a / b);
            }
        }
    }
    //    println!("{:?}", stack);

    stack.pop().unwrap()
}
