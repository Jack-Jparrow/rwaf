//! @Author       : 白银
//! @Date         : 2023-01-31 16:38:57
//! @LastEditors  : 白银
//! @LastEditTime : 2023-04-07 20:09:14
//! @FilePath     : /rwaf/src/module/protect/watch/watch_disk.rs
//! @Description  : 每个磁盘%util指标的平均%util all disks' avg %util
//! @Attention    :
//! @Copyright (c) 2023 by 白银 captain-jparrow@qq.com, All Rights Reserved.

use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_file_lines(file_path: &str) -> Vec<String> {
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);
    reader.lines().map(|line| line.unwrap()).collect()
}

pub fn output_get_disk_state() -> String {
    let lines = read_file_lines("/proc/diskstats"); // 读取/proc/diskstats文件中的所有行 Read all lines of the "/proc/diskstats" file

    let mut total_completed: u64 = 0; // 所有磁盘的I/O操作完成数之和 The total number of I/O completed on all disks
    let mut time_spent: u64 = 0; // 系统启动以来的总时间片数（ms） The total number of jiffies spent by the disk I/O scheduler since boot

    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect(); // 将行内容按空格分割为字符串片段，保存到parts中 Split the line by whitespace and store the resulting substrings in "parts"
                                                                  // 计算本行磁盘I/O完成数（包括读和写）之和 Calculate the number of I/O requests completed on this disk (both reads and writes)
        let read_completed: u64 = parts[3].parse().unwrap_or(0); // 该磁盘读取完成的请求数 Number of read requests completed on the disk
        let write_completed: u64 = parts[7].parse().unwrap_or(0); // 该磁盘写入完成的请求数 Number of write requests completed on the disk
        total_completed += read_completed + write_completed;
        time_spent += parts[13].parse::<u64>().unwrap_or(0) + parts[14].parse::<u64>().unwrap_or(0);
        // 计算系统启动以来磁盘I/O调度器处理所有请求所花费的时间（单位：ms） Calculate the total number of jiffies spent by the disk I/O scheduler on all disks since boot
    }

    let seconds = time_spent as f64 / 1000.0; // 计算总时间片数量所对应的时间（单位：s） Convert the total number of jiffies to seconds
    let avg_util = format!("{:.1}%", total_completed as f64 / seconds); // 计算平均%util值，并将格式化 Calculate the average %util of all disks, and format it as a string

    avg_util
}
