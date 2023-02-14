//! @Author       : 白银
//! @Date         : 2023-02-01 20:23:32
//! @LastEditors  : 白银
//! @LastEditTime : 2023-02-14 19:51:34
//! @FilePath     : /rwaf/src/module/restore/make_restore.rs
//! @Description  : 从备份服务器恢复
//! @Attention    :
//! @Copyright (c) 2023 by 白银 captain-jparrow@qq.com, All Rights Reserved.

use std::{fs::File, io::Read, process::Command};

pub fn start_make_restore() {
    let dst_path = get_only_dst_path();
    let src_ip = get_only_src_ip();
    let src_username = get_only_src_username();
    let src_path = get_only_src_path();

    let fin_shell = "scp -r ".to_string()
        + &src_username
        + &"@"
        + &src_ip
        + &":"
        + &src_path
        + &" "
        + &dst_path;
    // let fin_shell = "scp -r root@129.226.211.132:/home/lighthouse/pwn /home/jack/Desktop/pwm".to_string();

    // println!("{}", make_restore(fin_shell));
    let binding = get_date_time();
    let date_time: Vec<&str> = binding.split("\n").collect();
    let now_date = date_time.clone()[0]; //get system date
    let now_time = date_time.clone()[1]; //get system time

    make_restore(fin_shell);
    write_to_restore_log_sql();
}

// fn make_restore(fin_shell: String) {
//     execute::shell(fin_shell);
// }

fn make_restore(fin_shell: String) {
    let mut command = execute::shell(fin_shell);
    // let output = Command::new("echo").args(["123".to_string()]).output().unwrap();

    match command.status().unwrap().code() {
        Some(code) => {
            println!("Exit Status: {}", code);
        }
        None => {
            println!("Process terminated");
        }
    }
    // let res = String::from_utf8(output.stdout).unwrap();

    // res.trim().to_string()
}

fn get_date_time() -> String {
    // let mut command = execute::shell("echo $(date +%F%n%T)");
    let output = Command::new("date").arg("+%F%n%T").output().unwrap();
    let res = String::from_utf8(output.stdout).unwrap();

    res
}

//The dst_path here is the another_src_path of the config file
fn get_only_dst_path() -> String {
    let mut open_config = File::open("src/config").unwrap();
    let mut config_content = String::new();
    open_config.read_to_string(&mut config_content).unwrap();

    let binding = config_content;
    let res1: Vec<&str> = binding.split("\n").collect(); //get line
    let binding2 = res1.clone()[11]; //get line
    let binding3 = binding2.to_string();
    let res2: Vec<&str> = binding3.split("→").collect(); //get left
    let binding4 = res2.clone()[0]; //get left
    let binding5 = binding4.to_string();
    let real_res_tmp = get_needed_thing(&binding5); //get dst_path

    real_res_tmp.to_string()
}

//The src_ip here is the dst_ip of the config file
fn get_only_src_ip() -> String {
    let mut open_config = File::open("src/config").unwrap();
    let mut config_content = String::new();
    open_config.read_to_string(&mut config_content).unwrap();

    let binding = config_content;
    let res1: Vec<&str> = binding.split("\n").collect(); //get line
    let binding2 = res1.clone()[3]; //get line
    let binding3 = binding2.to_string();
    let res2: Vec<&str> = binding3.split("→").collect(); //get left
    let binding4 = res2.clone()[0]; //get left
    let binding5 = binding4.to_string();
    let real_res_tmp = get_needed_thing(&binding5); //get src_ip

    real_res_tmp.to_string()
}

//The src_username here is the dst_username of the config file
fn get_only_src_username() -> String {
    let mut open_config = File::open("src/config").unwrap();
    let mut config_content = String::new();
    open_config.read_to_string(&mut config_content).unwrap();

    let binding = config_content;
    let res1: Vec<&str> = binding.split("\n").collect(); //get line
    let binding2 = res1.clone()[4]; //get line
    let binding3 = binding2.to_string();
    let res2: Vec<&str> = binding3.split("→").collect(); //get left
    let binding4 = res2.clone()[0]; //get left
    let binding5 = binding4.to_string();
    let real_res_tmp = get_needed_thing(&binding5); //get src_username

    real_res_tmp.to_string()
}

//The src_path here is the dst_path of the config file
fn get_only_src_path() -> String {
    let mut open_config = File::open("src/config").unwrap();
    let mut config_content = String::new();
    open_config.read_to_string(&mut config_content).unwrap();

    let binding = config_content;
    let res1: Vec<&str> = binding.split("\n").collect(); //get line
    let binding2 = res1.clone()[5]; //get line
    let binding3 = binding2.to_string();
    let res2: Vec<&str> = binding3.split("→").collect(); //get left
    let binding4 = res2.clone()[0]; //get left
    let binding5 = binding4.to_string();
    let real_res_tmp = get_needed_thing(&binding5); //get src_path

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

fn write_to_restore_log_sql() {
    todo!()
}
