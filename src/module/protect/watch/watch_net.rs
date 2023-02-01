//! @Author       : 白银
//! @Date         : 2023-01-31 19:59:07
//! @LastEditors  : 白银
//! @LastEditTime : 2023-02-01 15:18:53
//! @FilePath     : /rwaf/src/module/protect/watch/watch_net.rs
//! @Description  : 获取带宽使用情况
//! @Attention    : 获取到两行信息，第一行开始，第二行结束，每行空格隔开前为发送，后为接收
//! @Copyright (c) 2023 by Jack-Jparrow captain-jparrow@qq.com, All Rights Reserved.

use std::process::Command;

pub fn output_get_net_state_receive() -> String {
    //行
    let _start = split_line1();
    let _end = split_line2();

    //列
    let _start_tmp: Vec<&str> = _start.split(" ").collect();
    let _end_tmp: Vec<&str> = _end.split(" ").collect();

    let res_receive_start: f64 = _start_tmp[0].trim().parse().unwrap();
    let res_receive_end: f64 = _end_tmp[0].trim().parse().unwrap();

    let period_time: f64 = 0.5; //watch_net.sh中sleep的时间，单位s

    //转换为Mbps，直接用Mbs的话不用*8
    let receive_res = (res_receive_end - res_receive_start) / 1024.0 / 1024.0 / period_time * 8.0;

    let res_receive = receive_res.to_string();

    let fuhao1 = ".";
    let fuhao2 = "Mbps";
    let res_receive_final =
        get_zhengshu(&res_receive).to_string() + &fuhao1 + get_xiaoshu(&res_receive) + &fuhao2;

    res_receive_final
}

pub fn output_get_net_state_send() -> String {
    //行
    let _start = split_line1();
    let _end = split_line2();

    //列
    let _start_tmp: Vec<&str> = _start.split(" ").collect();
    let _end_tmp: Vec<&str> = _end.split(" ").collect();

    let res_send_start: f64 = _start_tmp[1].trim().parse().unwrap();
    let res_send_end: f64 = _end_tmp[1].trim().parse().unwrap();

    let period_time: f64 = 0.5; //watch_net.sh中sleep的时间，单位s

    //转换为Mbps，直接用Mbs的话不用*8
    let send_res = (res_send_end - res_send_start) / 1024.0 / 1024.0 / period_time * 8.0;

    let res_send = send_res.to_string();

    let fuhao1 = ".";
    let fuhao2 = "Mbps";
    let res_send_final =
        get_zhengshu(&res_send).to_string() + &fuhao1 + get_xiaoshu(&res_send) + &fuhao2;

    res_send_final
}

fn get_net_state() -> String {
    let output = Command::new("bash")
        .arg("src/module/protect/watch/watch_net.sh")
        .output()
        .unwrap();
    let res = String::from_utf8(output.stdout).unwrap();

    res.trim().to_string()
}

fn split_line1() -> String {
    let binding = get_net_state();
    let res1: Vec<&str> = binding.split("\n").collect();
    // println!("{:?}", res1);
    let binding2 = res1.clone()[0];
    // let res2: Vec<&str> = binding2.clone().split(" ").collect();
    // println!("{:?}", res2);
    // res2
    binding2.to_string()
}

fn split_line2() -> String {
    let binding = get_net_state();
    let res1: Vec<&str> = binding.split("\n").collect();
    // println!("{:?}", res1);
    let binding2 = res1.clone()[1];
    // let res2: Vec<&str> = binding2.clone().split(" ").collect();
    // println!("{:?}", res2);
    // res2
    binding2.to_string()
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

fn get_xiaoshu(s: &String) -> &str {
    // let len = s.trim().chars().count();
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b'.' {
            return &s[i + 1..i + 3];
        }
    }

    // s.len()
    &s[..]
}
