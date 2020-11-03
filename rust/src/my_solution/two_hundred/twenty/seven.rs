use std::collections::HashMap;

///实现一个基本的计算器来计算一个简单的字符串表达式的值。
//
// 字符串表达式仅包含非负整数，+， - ，*，/ 四种运算符和空格  。 整数除法仅保留整数部分。
pub fn calculate(s: String) -> i32 {
    let chars = s.chars().collect::<Vec<_>>();

    let mut cur_number_str = String::new();
    let mut stack_number = vec![];
    let mut stack_symbol = vec![]; // * - /

    for i in &chars{
        match i {
            '0'..='9' =>{
                cur_number_str.push(*i);
            }
            '+'| '-' |'*' | '/'  => {
                stack_number.push(cur_number_str.parse::<i32>().unwrap());
                cur_number_str.clear();
                stack_symbol.push(*i);
            }
            _ => {
                //blank or other
                continue;
            }
        }
    }
    stack_number.push(cur_number_str.parse::<i32>().unwrap());
    // println!("{:?}",stack_number);
    // println!("{:?}",stack_symbol);
    let mut map = HashMap::new();
    for i in 0..chars.len(){
        map.insert(i,i);
    }
    let mut val = 0;
    for (k,v) in stack_symbol.iter().enumerate(){
        match v {
            '*' |'/' => {

                if *v == '*'{
                    val = stack_number[k] * stack_number[k+1];
                }else{
                    val = stack_number[k] / stack_number[k+1];
                }
                let map_val = *map.get(&k).unwrap();
                map.insert(k+1,map_val );
                stack_number[k+1] = val;
                stack_number[map_val] = val;
            }
            _ =>{}
        }
    }
    let mut res = stack_number[0];
    for (k,v) in stack_symbol.iter().enumerate(){
        match v {
            '+' => {
                res+= stack_number[k+1];
            }
            '-' =>{
                res-= stack_number[k+1];
            }
            _ =>{}
        }
    }
    res
}