/*
    Nth Fibonacci Number
    Implement a function to calculate the `n`th Fibonacci number. 
    The Fibonacci sequence is defined as follows:
    F(0) = 0, F(1) = 1, F(n) = F(n-1) + F(n-2) for n > 1.

    You need to implement the function `fib(n: i32) -> i32` to return the `n`th Fibonacci number.
    
    Hint: Consider using matrix exponentiation to solve the problem in O(log n) time complexity.
*/

use std::fmt::{self, Display, Formatter};
use std::collections::HashMap;
pub fn fib(n: i32) -> i32 {
    // TODO: Implement the logic to calculate the nth Fibonacci number using matrix exponentiation
    let mut hm = HashMap::<i32, i32>::new();
    hm.insert(0, 0);
    hm.insert(1, 1);
    fib2(n, &mut hm)
}

pub fn fib2(n: i32, hm: &mut HashMap<i32, i32>) -> i32 {
    if let Some(&v) = hm.get(&n) {
        return v;
    }

    let k1 = n - 1;
    let k2 = n - 2;
   
    let v1 = fib2(k1, hm);
    
    let v2 = fib2(k2, hm);
    

    let v = v1 + v2;
    hm.insert(n, v);

    return v;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_1() {
        let result = fib(0);
        println!("Fibonacci of 0: {}", result);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_fib_2() {
        let result = fib(1);
        println!("Fibonacci of 1: {}", result);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_fib_3() {
        let result = fib(2);
        println!("Fibonacci of 2: {}", result);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_fib_4() {
        let result = fib(3);
        println!("Fibonacci of 3: {}", result);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_fib_5() {
        let result = fib(10);
        println!("Fibonacci of 10: {}", result);
        assert_eq!(result, 55);
    }

    #[test]
    fn test_fib_6() {
        let result = fib(20);
        println!("Fibonacci of 20: {}", result);
        assert_eq!(result, 6765);
    }
}
