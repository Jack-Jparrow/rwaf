//! @Author       : 白银
//! @Date         : 2023-04-08 15:34:32
//! @LastEditors  : 白银
//! @LastEditTime : 2023-04-08 19:35:18
//! @FilePath     : /rwaf/src/module/protect/watch/watch_download_net.rs
//! @Description  : 获取下载带宽使用情况（Mbps） Get the download bandwidth usage (Mbps)
//! @Attention    : 机翻 machine translation
//! @Copyright (c) 2023 by 白银 captain-jparrow@qq.com, All Rights Reserved.

use std::fs;
use std::path::Path;
use std::{thread::sleep, time::Duration};

pub fn output_get_net_state_download() -> String {
    get_net_state_download(&get_network_device_names()[0])
}

// 获取下载带宽的占用情况并返回字符串 （Mbps） Get the download bandwidth usage information and return as a string (in Mbps).
pub fn get_net_state_download(device_name: &str) -> String {
    let sleep_time = Duration::from_secs_f32(1.0); // 定义等待时间 Definition of wait time
    let device_path = format!("/sys/class/net/{}/statistics/rx_bytes", device_name); //具体目录要根据网卡名称来 The specific directory should be based on the network card name

    // 读取系统网络下载字节数 Read the system's network download byte count
    let prev_bytes = match fs::read_to_string(&device_path) {
        Ok(content) => content.trim().parse::<u64>().unwrap_or_default(),
        Err(_) => return "Error: unable to read network statistics".to_owned(),
    };

    sleep(sleep_time); // 等待1秒钟 Wait for 1 second
    let curr_bytes = match fs::read_to_string(&device_path) {
        Ok(content) => content.trim().parse::<u64>().unwrap_or_default(),
        Err(_) => return "Error: unable to read network statistics".to_owned(),
    };

    let download_speed =
        ((curr_bytes - prev_bytes) as f64 * 8.0 / 1000.0 / 1000.0) / sleep_time.as_secs_f64();

    format!("{:.2}Mbps", download_speed)
}

// 获取网卡名称 get network device names
pub fn get_network_device_names() -> Vec<String> {
    let net_dir = Path::new("/sys/class/net");
    let mut names = vec![];
    if let Ok(entries) = fs::read_dir(net_dir) {
        for entry in entries.flatten() {
            if let Ok(name) = entry.file_name().into_string() {
                names.push(name);
            }
        }
    }
    names
}
