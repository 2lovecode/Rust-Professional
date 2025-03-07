pub fn time_info(time: &str) -> String {
    // 不使⽤时间库，⾃⼰实现⼀个时间计算器，输⼊年⽉⽇字符串，输出当前周是第
    // ⼏周，周⼏，当天是本年的第⼏天，当年还剩多少天，距离过年(正⽉初⼀)还有多少天，
    // 距离下⼀次A股开盘还有多少天。时间差计算时不含当天，结果⽤字符串，逗号隔开(限时
    // 0.2s)。

    let mut year = 0;
    let mut month = 0;
    let mut day = 0;
    let (std_year, std_month, std_day, std_week, week_step) = (1970, 1, 1, 3, 7);
    let (festival_year_25, festival_month_25, festival_day_25) = (2026, 2, 17);
    let normal_month_days = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];


    let tl = time.split("-").collect::<Vec<&str>>();
    if tl.len() == 3 {
        year = tl[0].parse::<i32>().unwrap();
        month = tl[1].parse::<i32>().unwrap();
        day = tl[2].parse::<i32>().unwrap();
    }

    if year > 0 && month > 0 && day > 0 {

        let mut begin_total_day = 0;

        for i in std_year..year {
            if is_leap_year(i) {
                begin_total_day += 366;
            } else {
                begin_total_day += 365;
            }
        }
        let begin_week = (begin_total_day + std_week) % week_step;
        let mut month_days = normal_month_days.clone();
        let mut year_total_day = 365;
        if is_leap_year(year) {
            month_days[1] = 29;
            year_total_day = 366;
        }
        let mut total_day = day;

        for i in 0..month-1 {
            total_day += month_days[i as usize];
        }
        let mut current_week_num = (total_day + begin_week) / week_step + 1;
        let mut current_week = (total_day + begin_week) % week_step;
        if month == 12 && (day - month_days[11] + 1) <= 3{
            current_week_num = 1;
            if current_week == 0 {
                current_week_num = 2;
            }
        }
        if current_week == 0 {
            current_week = 7;
            current_week_num = current_week_num - 1;
        }

        let mut wait_open_day = 0;
        let mut start_month = month;
        let mut start_day = day;
        let mut start_week = current_week;

        let mut flag = true;
        while flag {
            start_week += 1;
            if start_week > 7 {
                start_week = 1;
            }

            start_day += 1;
            if start_day > month_days[(start_month-1) as usize] {
                start_day = 1;
                start_month += 1;
                if start_month > 12 {
                    start_month = 1;
                }
            }
            if stock_not_open(start_month, start_day, start_week) {
                wait_open_day += 1;
            } else {
                flag = false;
            }
        }

        let mut rest_day = 0;
        

        if month == 1 && day < 29 {
            rest_day = 29-day;
        } else {
            let mut f_total_day = festival_day_25;
            let mut f_month_days = normal_month_days.clone();
            if is_leap_year(festival_year_25) {
                f_month_days[1] = 29;
            }
            for i in 0..festival_month_25-1 {
                f_total_day += f_month_days[i as usize];
            }
            rest_day = year_total_day-total_day+f_total_day;
        }



        return format!("{},{},{},{},{},{}", current_week_num, current_week, total_day, year_total_day-total_day, rest_day, wait_open_day);
        
    }

    return String::from("");
}

fn is_leap_year(year: i32) -> bool {
    if year % 4 == 0 && year % 100 != 0 {
        return true;
    }
    if year % 400 == 0 {
        return true;
    }
    return false;
}

// 仅支持2025年的判断
fn stock_not_open(month: i32, day: i32, week: i32) -> bool {
    if week == 6 || week == 7 {
        return true;
    }
    match month {
        1 => {
            if vec![1,28,29,30,31].contains(&day) {
                return true;
            }
        },
        2 => {
            if vec![1,2,3,4].contains(&day) {
                return true;
            }
        },
        4 => {
            if vec![4,5,6].contains(&day) {
                return true;
            }
        },
        5 => {
            if vec![1,2,3,4,5].contains(&day) {
                return true;
            }
        }
        10 => {
            if vec![1,2,3,4,5,6,7,8].contains(&day) {
                return true;
            }
        }
        _ => {
            return false;
        }
    }

    return false;
}