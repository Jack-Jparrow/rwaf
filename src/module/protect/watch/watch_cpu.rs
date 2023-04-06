//! @Author       : 白银
//! @Date         : 2023-01-30 19:04:47
//! @LastEditors  : 白银
//! @LastEditTime : 2023-04-06 18:36:36
//! @FilePath     : /rwaf/src/module/protect/watch/watch_date_time_cpu.rs
//! @Description  : cpu占用率% CPU_usage%
//! @Attention    :
//! @Copyright (c) 2023 by 白银 captain-jparrow@qq.com, All Rights Reserved.

use std::{
    fs::File,
    io::Read
};

// 开始获取CPU占用率
// CPU_usage
pub fn output_get_cpu_state() -> String {
    // 打开 /proc/stat 文件，获取文件句柄 Open /proc/stat file and get the file handle
    let mut file = match File::open("/proc/stat") {
        Ok(file) => file, // 文件打开成功，则使用文件句柄 If the file opens successfully, use the file handle
        Err(_) => return "failed to open /proc/stat".to_string(), // 打开失败，则返回一个错误信息 If the file fails to open, return an error message
    };

    // 读取文件内容到字符串中 Read the contents of the file into a string
    let mut content = String::new();
    if let Err(_) = file.read_to_string(&mut content) {
        return "failed to read /proc/stat".to_string(); // 如果读取失败，则返回一个错误信息 If reading the file contents fails, return an error message
    }

    // 将内容按行分割为字符串向量 Split the contents of the file into lines
    let lines: Vec<&str> = content.lines().collect();

    // 遍历所有行，找到第一行以 "cpu" 开头的行 Iterate through each line until the first line that starts with "cpu" is found
    for line in lines {
        let tokens: Vec<&str> = line.split_whitespace().collect(); // 将行字符串按空格分割成一个向量 Split the line into tokens by whitespace
        if tokens.len() > 0 && tokens[0] == "cpu" {
            // 判断第一个字符串是否为"cpu" Check if the first token is "cpu"
            // 解析各个参数的值 Extract values for various parameters
            let user = tokens[1].parse::<f64>().unwrap_or(0.0); // 解析出用户进程占用CPU时间 Parse the amount of CPU time used by user processes
            let nice = tokens[2].parse::<f64>().unwrap_or(0.0); // 解析出nice值，表示出于调度优先级，进程在内核中占用的CPU时间 Parse the nice value, which represents the amount of CPU time spent by a process in the kernel due to its scheduling priority
            let system = tokens[3].parse::<f64>().unwrap_or(0.0); // 解析系统态时间并转换为浮点数类型 Parse the amount of CPU time spent in system mode and convert it to a floating-point number
            let idle = tokens[4].parse::<f64>().unwrap_or(0.0); // 解析空闲时间并转换为浮点数类型 Parse the amount of idle CPU time and convert it to a floating-point number
            let iowait = tokens[5].parse::<f64>().unwrap_or(0.0); // 解析出等待I/O操作的CPU时间 Parse the amount of CPU time spent waiting for I/O operations and convert it to a floating-point number
            let irq = tokens[6].parse::<f64>().unwrap_or(0.0); // 获取硬中断时间并转换为浮点数类型 Get the amount of time spent in hardware interrupts and convert it to a floating-point number
            let softirq = tokens[7].parse::<f64>().unwrap_or(0.0); // 获取软中断时间并转换为浮点数类型 Get the amount of time spent in software interrupts and convert it to a floating-point number
            let steal = tokens[8].parse::<f64>().unwrap_or(0.0); // 获取steal时间并转换为浮点数类型 Get the amount of time stolen from a virtual machine and convert it to a floating-point number
            let guest = tokens[9].parse::<f64>().unwrap_or(0.0); // 获取guest时间并转换为浮点数类型Get the amount of time spent running a virtual CPU for guest operating systems and convert it to a floating-point number
            let guest_nice = tokens[10].parse::<f64>().unwrap_or(0.0); // 获取 uest_nice时间并转换为浮点数类型 Get the amount of nice CPU time spent running a virtual CPU for guest operating systems and convert it to a floating-point number

            // 计算CPU负载 Calculate CPU load
            let total_time =
                user + nice + system + idle + iowait + irq + softirq + steal + guest + guest_nice; // 计算总的CPU时间 Calculate the total CPU time
            let load = (total_time - idle) / total_time * 100.0; // 计算占用率百分比 Calculate the CPU load as a percentage of non-idle time

            return format!("{:.1}%", load); // 将负载值格式化成字符串并返回 Format the CPU load as a string and return it
        }
    }

    // 如果无法获取负载值，则返回相应的错误信息 If the CPU load cannot be obtained, return an error message
    "failed to get CPU load".to_string()
}

