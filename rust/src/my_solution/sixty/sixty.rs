pub fn get_permutation(n: i32, k: i32) -> String {
    let mut k = k;
    let mut n = n;
    let mut factor = 1;
    let mut chars = vec![];
    for i in 1..=n {
        factor *= i;
        chars.push(std::char::from_digit(i as u32, 10).unwrap());
    }

    let mut result = String::new();
    while k != 1 {
        if factor == k {
            for i in chars.iter().rev() {
                result.push(*i);
            }
            return result;
        }
        factor /= n;
        n -= 1;
        let quotient = (k - 1) / factor;

        result.push(chars.remove(quotient as usize));

        k -= quotient * factor;

        //        println!(
        //            "k = {}, n = {}, factor = {}, quotient = {}, result = {:?}",
        //            k, n, factor, quotient, result
        //        );
    }
    for i in chars.iter() {
        result.push(*i);
    }
    result
}
