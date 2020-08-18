pub fn divide(dividend: i32, divisor: i32) -> i32 {
    if dividend == 0 {
        return 0;
    }

    if divisor == 1 {
        return dividend;
    }
    if dividend == std::i32::MIN && divisor == -1 {
        return std::i32::MAX;
    }

    if divisor == -1 {
        return -dividend;
    }

    let mut dividend = dividend;
    if (dividend > 0 && divisor > 0) {
        return divide(-dividend, -divisor);
    } else if dividend > 0 && divisor < 0 {
        return -divide(-dividend, divisor);
    } else if dividend < 0 && divisor > 0 {
        return -divide(dividend, -divisor);
    }
    let mut result = 0;
    while dividend <= divisor {
        let mut for_result = 1;
        let mut for_sub: i64 = divisor as i64;
        while for_sub + for_sub >= dividend as i64 {
            for_sub += for_sub;
            for_result += for_result;
        }
        dividend -= for_sub as i32;
        result += for_result;
    }
    result
}
