pub fn retire_time(time: &str, tp: &str) -> String {
    // 实现⼀个退休计算器，要求输⼊出⽣年⽉、⼈员类型，输出退休时间、退休年龄
    // (精确到2位⼩数)和延迟退休⽉数。(请结合中央最新政策，后续延迟退休政策出台时，代
    // 码改动要⼩或⽆改动)(限时0.2s)。

    let mut old_age = 60f32;
    let mut new_age = 63f32;
    let mut step = 4;
    let start = 2025;
    let mut end = 2040;

    let mut new_year = 0;
    let mut new_month = 0;
    let mut total_month = 0;
    
    
    match tp {
        "男职工" => {
            old_age = 60f32;
            new_age = 63f32;
            step = 4;
            end = 2040;
        },
        "原法定退休年龄55周岁女职工" => {
            old_age = 55f32;
            new_age = 58f32;
            step = 4;
            end = 2040;
        },
        "原法定退休年龄50周岁女职工" => {
            old_age = 50f32;
            new_age = 55f32;
            step = 2;
            end = 2035;
        },
        _ => {

        }
    }

    let tl: Vec<&str> = time.split("-").collect();

    let mut b_year = 0;
    let mut b_month = 0;

    if tl.len() == 2 {
        b_year = tl[0].parse::<i32>().unwrap();
        b_month = tl[1].parse::<i32>().unwrap();
    } 
    if b_year > 0 && b_month > 0 {
        let old_year = b_year + old_age as i32;
        let old_month = b_month;

        if old_year >= end {
            new_year = b_year + new_age as i32;
            new_month = old_month;
            total_month = (new_age - old_age) as i32 * 12;
        } else if old_year >= start {
            let total = (old_year - start) * 12 + old_month;
            if total % step != 0 {
                total_month = total / step + 1;
            } else {
                total_month = total / step;
            }
            new_month = (old_month + total_month) % 12;
            new_year = old_year + (old_month + total_month) / 12;
            new_age = old_age + total_month as f32 / 12f32;
        } else {
            new_year = old_year;
            new_month = old_month;
            new_age = old_age;
            total_month = 0;
        }
    }

    let mut new_age_str = format!("{:.2}", new_age);
    if new_age_str.ends_with(".00") {
        new_age_str = format!("{:.0}", new_age)
    }
    let mut new_month_str = format!("{}", new_month);
    if new_month < 10 {
        new_month_str = format!("0{}", new_month);
    }

    format!("{}-{},{},{}", new_year, new_month_str, new_age_str, total_month)
}
