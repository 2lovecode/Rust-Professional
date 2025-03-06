pub fn convert_base(num_str: &str, to_base: u32) -> String {
    // 解析输入
    let (firt, rest) = num_str.split_once('(').unwrap();
    let sec = rest.strip_suffix(')').unwrap();

    let sec_num = sec.parse::<i32>().unwrap();

    // N进制转10进制
    let dec = firt.chars().rev().enumerate().fold(0, |acc, (i, c)| {
        let digit = c.to_digit(sec_num as u32).unwrap() as i32;
        acc + digit * sec_num.pow(i as u32)
    });

    
    // 10进制转N进制
    let mut ss = String::from("");

    let mut dec = dec;
    while dec > 0 {
        let ys = dec % to_base as i32;
        ss.push(char::from_digit(ys as u32, to_base as u32).unwrap());
        dec = dec / to_base as i32;
    }

    return ss.chars().rev().collect();
}
