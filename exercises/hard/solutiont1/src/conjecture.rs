
pub fn goldbach_conjecture() -> String  {
	let mut prime_vec:Vec<u64> = Vec::new();
	let mut result_vec:Vec<u64> = Vec::new();
	let mut temp:u64 = 3;

	prime_vec.push(2);
	while result_vec.len() < 2 {
		if is_prime(temp) {
			prime_vec.push(temp);
		} else {
			let mut index:usize = 0;
			let mut flag:bool = true;
			while index < prime_vec.len() && temp > prime_vec[index] {
				let mut tmp:u64 = 1;
				while tmp * tmp * 2 + prime_vec[index] < temp {
					tmp += 1;
				}
				if tmp * tmp * 2 + prime_vec[index] == temp {
					flag = false;
					break;
				}
				index += 1;
			}
			if flag {
				result_vec.push(temp);
			}
		}

		temp += 2;
	}

	return format!("{},{}",result_vec[0],result_vec[1]);
}

fn is_prime(number:u64) -> bool {
	let stop:u64 = number / 2;
	let mut temp:u64 = 2;
	while temp < stop  {
		if number % temp == 0 {
			return false;
		}
		temp += 1;
	}
	return true;
}
