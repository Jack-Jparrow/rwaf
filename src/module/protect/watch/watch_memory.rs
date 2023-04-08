//! @Author       : 白银
//! @Date         : 2023-01-29 21:19:15
//! @LastEditors  : 白银
//! @LastEditTime : 2023-04-08 15:23:56
//! @FilePath     : /rwaf/src/module/protect/watch/watch_memory.rs
//! @Description  : 获取内存占用% mem_usage%
//! @Attention    :
//! @Copyright (c) 2023 by 白银 captain-jparrow@qq.com, All Rights Reserved.

use std::fs;

// 从/proc/meminfo中获取与给定键关联的值，如果未找到则返回None Get the value associated with a key in /proc/meminfo, returning None if not found
fn get_mem_value(key: &str) -> Option<i32> {
    fs::read_to_string("/proc/meminfo") // 读取meminfo文件并返回其内容的字符串 Read the contents of the meminfo file as a string
        .ok() // 如果文件读取失败，read_to_string()方法会返回Err类型的错误，这时候会返回None If the file read fails, the read_to_string() method returns an error of type Err, which returns None in this case
        .and_then(|contents| {
            // 如果成功读取到文件，则调用and_then()方法来对返回的结果进行处理，用了闭包 Use and_then() method to handle the returned result if reading the file succeeds, using a closure
            contents.lines().find_map(|line| {
                // 对字符串进行迭代，查找与指定键相关的条目，并找出第一个匹配项 Iterate through the string to find the entry related to the specified key, and find the first match
                let parts: Vec<&str> = line.split(':').collect(); // 将条目拆分为键值对，以“:”作为分隔符 Split the entry into a key-value pair using ':' as the delimiter
                if parts[0].trim() == key {
                    // 如果找到了与指定键匹配的项，则解析其值，否则返回None If a matching entry is found for the specified key, parse its value and return it; otherwise return None
                    parts[1]
                        .trim()
                        .split(' ')
                        .next()
                        .and_then(|val| val.parse().ok())
                } else {
                    None
                }
            })
        })
}

// 计算当前内存使用率的百分比 Calculate the current memory usage percentage
fn calc_mem_usage_percent() -> Option<f64> {
    let total_mem = get_mem_value("MemTotal")?; // 获取物理内存总量，如果获取失败则返回None Get total physical memory; if the operation fails, return None
    let available_mem = get_mem_value("MemAvailable")?; // 获取可用内存，如果获取失败则返回None Get available memory; if the operation fails, return None
    Some(100.0 - ((available_mem as f64 / total_mem as f64) * 100.0)) // 计算并返回使用率百分比 Calculate and return the memory usage percentage
}

// 转string to_string
pub fn output_get_mem_state() -> String {
    match calc_mem_usage_percent() {
        // 调用calc_mem_usage_percent()方法来获取当前内存使用率，并与匹配模式进行匹配 Call the calc_mem_usage_percent() function to get the current memory usage percentage and use matching patterns to match it
        Some(usage_percent) => format!("{:.1}%", usage_percent), // 如果匹配到了某个使用率百分比，则将其格式化为字符串并返回 If a memory usage percentage is matched, format it as a string and return
        None => "N/A".to_string(), // 如果没有匹配到任何使用率百分比，则返回字符串“N/A” If no memory usage percentage is matched, return the string "N/A"
    }
}
