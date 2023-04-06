//! @Author       : 白银
//! @Date         : 2023-04-06 19:34:04
//! @LastEditors  : 白银
//! @LastEditTime : 2023-04-06 19:47:29
//! @FilePath     : /rwaf/src/module/protect/watch/watch_time.rs
//! @Description  : 开始获取系统时间，格式为 "hh:mm:ss" time, in the format "hh:mm:ss"
//! @Attention    :
//! @Copyright (c) 2023 by 白银 captain-jparrow@qq.com, All Rights Reserved.

use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn output_get_time_state() -> String {
    let now = SystemTime::now(); // 获取当前时间 Get the current time
    let since_epoch = now.duration_since(UNIX_EPOCH).unwrap().as_secs(); // 计算自Unix纪元以来经过的秒数 Calculate the number of seconds since Unix epoch
    let secs_in_day = 60 * 60 * 24; // 每天的秒数
    let secs_since_midnight = Duration::from_secs(since_epoch % secs_in_day); // 计算自午夜以来经过的秒数 Calculate the number of seconds since midnight
    let (h, m, s) = (
        secs_since_midnight.as_secs() / 3600 + 8, // 小时数，东八区 Calculate the number of hours, UTC+8
        (secs_since_midnight.as_secs() / 60) % 60, // 分钟数 Calculate the number of minutes
        secs_since_midnight.as_secs() % 60,       // 秒数 Calculate the number of seconds
    );

    format!("{:02}:{:02}:{:02}", h, m, s)
}
