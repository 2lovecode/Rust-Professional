
use std::collections::HashMap;
pub fn goldbach_conjecture() -> String {
    //

    let mut res: Vec<String> = Vec::new();
    let mut primes: Vec<i32> = Vec::new();
    let mut sqrts: HashMap<i32, i32> = HashMap::new();

    let mut current = 1;
    let mut cnt = 0;
    while cnt < 2 {
        if current >= std::i32::MAX {
            break;
        }

        if current < (std::i32::MAX as f64).sqrt() as i32 {
            sqrts.insert(current*current, current);
        }

        if is_prime(current) {
            primes.push(current);
        } else if is_odd(current) {
            let mut has = false;
            for i in 0..primes.len() {
                if primes[i] > current {
                    break;
                }
                let re = current - primes[i];
                let rr = re/2;
                if re % 2 == 0 && sqrts.contains_key(&rr) {
                    has = true;
                    break;
                }
            }
            if !has && current != 1{
                let  cs = current.to_string();
                res.push(cs);
                cnt += 1;
            }
        }

        current += 1;
    }
    res.join(",")
}

fn is_odd(n: i32) -> bool {
    n % 2 != 0
}

fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=(n as f64).sqrt() as i32 {
        if n % i == 0 {
            return false;
        }
    }
    true
}
