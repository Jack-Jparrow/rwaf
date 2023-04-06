//! @Author       : 白银
//! @Date         : 2023-04-06 19:29:05
//! @LastEditors  : 白银
//! @LastEditTime : 2023-04-06 19:29:47
//! @FilePath     : /rwaf/src/module/protect/watch/watch_date.rs
//! @Description  : 开始获取系统日期，格式为 "yyyy-dd-mm" date, in the format "yyyy-dd-mm"
//! @Attention    : 
//! @Copyright (c) 2023 by 白银 captain-jparrow@qq.com, All Rights Reserved. 

use std::time::{SystemTime, UNIX_EPOCH};

pub fn output_get_date_state() -> String {
    let now = SystemTime::now(); // 获取当前时间 Get the current time
    let since_epoch = now.duration_since(UNIX_EPOCH).unwrap().as_secs() + 3600 * 8; // 获取自Unix纪元以来经过的秒数，东八区+8小时 Get the number of seconds that have elapsed since the Unix epoch, UTC+8
    let secs_in_day: u64 = 60 * 60 * 24; // 一天中的秒数 The number of seconds in a day
    let days_since_epoch: u64 = since_epoch / secs_in_day; // 将秒数转换为天数 Convert the number of elapsed seconds to the number of elapsed days
    let ymd_days: i32 = (days_since_epoch + 719468) as i32; // 天数转化为从公元1年1月1日至今的天数 Convert the elapsed days to days since January 1, 1 (start of the Gregorian calendar)
    let era: i32 = if ymd_days >= 0 {
        // 计算年代数 Calculate the "era" (number of 400-year blocks since January 1, 1)
        ymd_days / 146097 // 每400年（146097天）为一个纪元 146097 days are in each era (400 x 365 + 97 leap days)
    } else {
        (ymd_days - 146096) / 146097
    };
    let doe: i32 = ymd_days - era * 146097; // 每个纪元的第一天 Calculate the day-of-era (number of days since the start of the current era)
    let yoe: usize = (doe as f64 / 365.2425).floor() as usize; // 当前纪元的年数 Calculate the year-of-era (number of years since the start of the current era)
    let y_and_doe_difference: u64 = (doe as u64) - (365 * yoe as u64) - (yoe as u64 / 4) // 从年首到当前日期的天数，关键步骤 Calculate the number of days between the start of the year and the current date, critical step
        + (yoe as u64 / 100)
        - (yoe as u64 / 400);
    let mp: u64 = (5 * y_and_doe_difference + 2) / 153; // 计算月份 Calculate the "month parameter"
    let d: u64 = y_and_doe_difference - (153 * mp + 2) / 5 + 1; // 当前月的日期 Calculate the day-of-month
    let m: u64 = mp + if mp < 10 { 3 } else { 9 }; // 当前月份 Calculate the month number
    let y: u64 = (era as u64 * 400) + (yoe as u64) - (if m <= 2 { 1 } else { 0 }); // 当前年份 Calculate the year number

    format!("{:04}-{:02}-{:02}", y, m, d)
}