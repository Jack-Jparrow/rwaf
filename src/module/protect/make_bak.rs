//! @Author       : 白银
//! @Date         : 2023-02-01 19:41:02
//! @LastEditors  : 白银
//! @LastEditTime : 2023-02-14 15:36:31
//! @FilePath     : /rwaf/src/module/protect/make_bak.rs
//! @Description  : 备份文件
//! @Attention    :
//! @Copyright (c) 2023 by 白银 captain-jparrow@qq.com, All Rights Reserved.

use std::{thread, time::Duration, fs::File, io::Read};

pub fn use_start_make_bak() {
    loop {
        let t = thread::spawn(move || start_make_bak());

        thread::sleep(Duration::from_secs(5184000)); //24h备份一次
    }
}

pub fn start_make_bak() {
    let src_path = get_only_src_path();
    let dst_ip = get_only_dst_ip();
    let dst_username = get_only_dst_username();
    let dst_path = get_only_dst_path();

    let fin_shell =
        "scp -r ".to_string() + &src_path + &" " + &dst_username + &"@" + &dst_ip + &":" + &dst_path;

    make_bak(fin_shell);
}

fn make_bak(fin_shell: String) {
    let mut command = execute::shell(fin_shell);
}

fn get_only_src_path() -> String {
    let mut open_config = File::open("src/config").unwrap();
    let mut config_content = String::new();
    open_config.read_to_string(&mut config_content).unwrap();

    let binding = config_content;
    let res1: Vec<&str> = binding.split("\n").collect(); //get line
    let binding2 = res1.clone()[2]; //get line
    let binding3 = binding2.to_string();
    let res2: Vec<&str> = binding3.split("→").collect(); //get left
    let binding4 = res2.clone()[2]; //get left
    let binding5 = binding4.to_string();
    let real_res_tmp = get_needed_thing(&binding5); //get src_path

    real_res_tmp.to_string()
}

fn get_only_dst_ip() -> String {
    let mut open_config = File::open("src/config").unwrap();
    let mut config_content = String::new();
    open_config.read_to_string(&mut config_content).unwrap();

    let binding = config_content;
    let res1: Vec<&str> = binding.split("\n").collect(); //get line
    let binding2 = res1.clone()[3]; //get line
    let binding3 = binding2.to_string();
    let res2: Vec<&str> = binding3.split("→").collect(); //get left
    let binding4 = res2.clone()[3]; //get left
    let binding5 = binding4.to_string();
    let real_res_tmp = get_needed_thing(&binding5); //get dst_ip

    real_res_tmp.to_string()
}

fn get_only_dst_username() -> String {
    let mut open_config = File::open("src/config").unwrap();
    let mut config_content = String::new();
    open_config.read_to_string(&mut config_content).unwrap();

    let binding = config_content;
    let res1: Vec<&str> = binding.split("\n").collect(); //get line
    let binding2 = res1.clone()[4]; //get line
    let binding3 = binding2.to_string();
    let res2: Vec<&str> = binding3.split("→").collect(); //get left
    let binding4 = res2.clone()[4]; //get left
    let binding5 = binding4.to_string();
    let real_res_tmp = get_needed_thing(&binding5); //get dst_username

    real_res_tmp.to_string()
}

fn get_only_dst_path() -> String {
    let mut open_config = File::open("src/config").unwrap();
    let mut config_content = String::new();
    open_config.read_to_string(&mut config_content).unwrap();

    let binding = config_content;
    let res1: Vec<&str> = binding.split("\n").collect(); //get line
    let binding2 = res1.clone()[5]; //get line
    let binding3 = binding2.to_string();
    let res2: Vec<&str> = binding3.split("→").collect(); //get left
    let binding4 = res2.clone()[5]; //get left
    let binding5 = binding4.to_string();
    let real_res_tmp = get_needed_thing(&binding5); //get dst_path

    real_res_tmp.to_string()
}

fn get_needed_thing(s: &String) -> &str {
    let len = s.trim().chars().count();
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b'=' {
            return &s[i + 1..len];
        }
    }

    // s.len()
    &s[..]
}