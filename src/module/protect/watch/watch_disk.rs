//! @Author       : 白银
//! @Date         : 2023-01-31 16:38:57
//! @LastEditors  : 白银
//! @LastEditTime : 2023-01-31 21:14:23
//! @FilePath     : /rwaf/src/module/protect/watch/watch_disk.rs
//! @Description  : 磁盘%util指标
//! @Attention    : ioRate计算方法：https://zhuanlan.zhihu.com/p/60000317
//! @Copyright (c) 2023 by 白银 captain-jparrow@qq.com, All Rights Reserved. 

use std::process::Command;

pub fn output_get_disk_state() -> String{
    let receive = get_disk_state();
    let res_tmp: Vec<&str> = receive.split("\n").collect();
    let start: f64 = res_tmp[0].trim().parse().unwrap();
    let end: f64 = res_tmp[1].trim().parse().unwrap();
    let period = 200.0;//watch_disk.sh中sleep的时间（此处为ms，sh文件中为s）
    let ioRate: f64 = (end - start) / period;
    let fuhao = "%";

    let ioRate_tmp = ioRate.to_string();
    let ioRate_res = get_zhengshu(&ioRate_tmp).to_string();

    let res = ioRate_res + &fuhao;

    res
    // ioRate.to_string()
    // ((end - start)).to_string()
}

fn get_disk_state() -> String {
    let output = Command::new("bash").arg("src/module/protect/watch/watch_disk.sh").output().unwrap();
    let res = String::from_utf8(output.stdout).unwrap();

    res.trim().to_string()
}

fn get_zhengshu(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b'.' {
            // return i;
            return &s[0..i];
        }
    }

    // s.len()
    &s[..]
}