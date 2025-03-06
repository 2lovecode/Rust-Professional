pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    // TODO: 这里写逻辑
    let mut fib1 = 0;
    let mut fib2 = 1;
    let mut sum: u32 = 0;

    while fib2 <= threshold {
        if fib2 % 2 != 0 {
            sum += fib2;
        }
        let fib22 = fib1 + fib2;
        fib1 = fib2;
        fib2 = fib22;
    }

    sum
}


