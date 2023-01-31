//! @Author       : 白银
//! @Date         : 2023-01-30 19:04:47
//! @LastEditors  : 白银
//! @LastEditTime : 2023-01-31 20:18:44
//! @FilePath     : /rwaf/src/module/protect/watch/watch_date_time_cpu.rs
//! @Description  : cpu占用率
//! @Attention    : 获取到三行信息，要最后一行，awk结果再去空格切片
//! @Copyright (c) 2023 by ${git_name_email}, All Rights Reserved.

use std::process::{Command, Stdio};

pub fn output_get_cpu_state() -> String {
    let receive = split_from_res_date_time_cpu_state();
    let res_tmp: Vec<&str> = receive.split(" ").collect();
    let cpu_tmp: f64 = res_tmp[0].trim().parse().unwrap();
    let cpu: f64 = 100.0 - cpu_tmp;
    let fuhao = "%";

    let cpu_res = cpu.to_string() + &fuhao;

    cpu_res.to_string()
}

pub fn output_get_date_state() -> String {
    let receive = split_from_res_date_time_cpu_state();
    let res_tmp: Vec<&str> = receive.split(" ").collect();
    let day = res_tmp[1];
    day.to_string()
}

pub fn output_get_time_state() -> String {
    let receive = split_from_res_date_time_cpu_state();
    let res_tmp: Vec<&str> = receive.split(" ").collect();
    let time = res_tmp[2];
    time.to_string()
}

fn split_from_res_date_time_cpu_state() -> String {
    let binding = res_date_time_cpu_state();
    let res1: Vec<&str> = binding.split("\n").collect();
    // println!("{:?}", res1);
    let binding2 = res1.clone()[2];
    // let res2: Vec<&str> = binding2.clone().split(" ").collect();
    // println!("{:?}", res2);
    // res2
    binding2.to_string()
}

fn res_date_time_cpu_state() -> String {
    let output1 = Command::new("vmstat")
        .arg("1")
        .arg(" 2")
        .arg("-t")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    // let output2 = Command::new("grep").arg("Mem").stdin(output1.stdout.unwrap()).stdout(Stdio::piped()).spawn().unwrap();
    let output3 = Command::new("awk")
        .arg("{print $15, $18, $19}")
        .stdin(output1.stdout.unwrap())
        .output()
        .unwrap();
    let res = String::from_utf8(output3.stdout).unwrap();

    res.trim().to_string()
}
