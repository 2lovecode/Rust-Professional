pub fn new_birthday_probability(n: u32) -> f64 {
    if n > 356 {
        return 1.0;
    }

    let mut gl: f64 = 1.0;
    for i in 0..n {
        
            gl = gl * ((365-i) as f64 / 365f64);
        

    }
    1.0 - gl
}
