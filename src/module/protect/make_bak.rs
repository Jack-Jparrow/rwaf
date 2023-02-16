//! @Author       : 白银
//! @Date         : 2023-02-01 19:41:02
//! @LastEditors  : 白银
//! @LastEditTime : 2023-02-16 18:29:06
//! @FilePath     : /rwaf/src/module/protect/make_bak.rs
//! @Description  : 备份文件
//! @Attention    :
//! @Copyright (c) 2023 by 白银 captain-jparrow@qq.com, All Rights Reserved.

use std::{fs::File, io::Read, process::Command, thread, time::Duration};

pub fn use_start_make_bak() {
    loop {
        let t = thread::spawn(move || start_make_bak());
        let binding = get_date_time();
        let date_time: Vec<&str> = binding.split("\n").collect();

        let now_date = date_time.clone()[0]; //get system date
        let now_time = date_time.clone()[1]; //get system time
        let do_what = "make bak";
        let event_id: String = now_date.to_string() + now_time + do_what;
        let input_event_id = super::super::use_sm3::sm3_main(event_id);

        // write_to_bak_log_sql();

        thread::sleep(Duration::from_secs(5184000)); //do every 24h
    }
}

pub fn start_make_bak() {
    let src_path = get_only_src_path();
    let dst_ip = get_only_dst_ip();
    let dst_username = get_only_dst_username();
    let dst_path = get_only_dst_path();

    let fin_shell = "scp -r ".to_string()
        + &src_path
        + &" "
        + &dst_username
        + &"@"
        + &dst_ip
        + &":"
        + &dst_path;

    make_bak(fin_shell);
}

fn make_bak(fin_shell: String) {
    let mut command = execute::shell(fin_shell);
    let a = command.output();
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
    let binding4 = res2.clone()[0]; //get left
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
    let binding4 = res2.clone()[0]; //get left
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
    let binding4 = res2.clone()[0]; //get left
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
    let binding4 = res2.clone()[0]; //get left
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

fn get_date_time() -> String {
    // let mut command = execute::shell("echo $(date +%F%n%T)");
    let output = Command::new("date").arg("+%F%n%T").output().unwrap();
    let res = String::from_utf8(output.stdout).unwrap();

    res
}

fn write_to_bak_log_sql() {
    todo!()
}
