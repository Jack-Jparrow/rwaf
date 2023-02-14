//! @Author       : 白银
//! @Date         : 2023-02-12 18:56:47
//! @LastEditors  : 白银
//! @LastEditTime : 2023-02-14 15:31:50
//! @FilePath     : /rwaf/src/module/warning/port_sql_xss.rs
//! @Description  :
//! @Attention    :
//! @Copyright (c) 2023 by 白银 captain-jparrow@qq.com, All Rights Reserved.

use execute::Execute;
use regex::Regex;
use std::{
    fs::{self, File},
    io::{BufRead, BufReader, Read, Write},
    path::Path,
    process::Command,
};

pub fn port_sql_xss_main() {
    let url = get_only_url();

    let port_scan_script = "src/module/warning/port_scan.py".to_string();
    let xss_script = "src/module/warning/xss.py".to_string();

    start_rm();

    println!("......Port scan is working, please wait......");
    port_scan(port_scan_script);
    let port_scan_script_log = "src/module/warning/.log/port.log".to_string();
    let port_scan_script_log_size = fs::metadata("src/module/warning/.log/port.log")
        .unwrap()
        .len();
    println!("***Port scan is done***\n");
    if port_scan_script_log_size > 0 {
        read_log(port_scan_script_log);

        println!("!!!Please confirm whether some ports are in use!!!\n")
    }
    println!("------------------------------------------------------------------------------\n");

    println!("......Sql injection test is working, please wait......");
    // check_sql_injection(sql_script, &url);
    let sql_script_log = "src/module/warning/.log/sql.log".to_string();
    use_sqlmap(&url, &sql_script_log);

    let sql_script_log_size = fs::metadata("src/module/warning/.log/sql.log")
        .unwrap()
        .len();
    println!("***Sql injection test is done***");
    if sql_script_log_size > 0 {
        read_log(sql_script_log); //read&output details

        println!("!!!There is a risk of sql injection!!!\n")
    } else {
        println!("@@@There is no risk of sql injection@@@\n")
    }
    println!("------------------------------------------------------------------------------\n");

    println!("......Xss test is working, please wait......");
    check_xss(xss_script, &url);
    let xss_script_log = "src/module/warning/.log/xss.log".to_string();
    let xss_script_log_size = fs::metadata("src/module/warning/.log/xss.log")
        .unwrap()
        .len();
    println!("***Xss test is done***");
    if xss_script_log_size > 0 {
        read_log(xss_script_log);

        println!("!!!There is a risk of xss!!!\n")
    }
    println!("------------------------------------------------------------------------------");
}

fn start_rm() {
    //delete port_scan log
    if Path::new("src/module/warning/.log/port.log").exists() {
        rm_rf_log("src/module/warning/.log/port.log".to_string());
    }

    //delete sql log
    if Path::new("src/module/warning/.log/sql.log").exists() {
        rm_rf_log("src/module/warning/.log/sql.log".to_string());
    }

    //delete sql_tmp log
    if Path::new("src/module/warning/.log/sql_tmp.log").exists() {
        rm_rf_log("src/module/warning/.log/sql_tmp.log".to_string());
    }

    //delete xss log
    if Path::new("src/module/warning/.log/xss.log").exists() {
        rm_rf_log("src/module/warning/.log/xss.log".to_string());
    }
}

fn rm_rf_log(log_path: String) {
    let mut command = execute::command_args!("rm", "-rf", log_path);
    let _output = command.execute_output().unwrap();
}

fn port_scan(port_scan_script: String) {
    let mut command = execute::command_args!("python3".to_string(), port_scan_script);
    let _output = command.execute_output().unwrap();
}

fn check_xss(xss_script: String, url: &str) {
    let mut command = execute::command_args!("python3".to_string(), xss_script, url.to_string());
    let _output = command.execute_output().unwrap();
}

fn get_only_url() -> String {
    let mut open_config = File::open("src/config").unwrap();
    let mut config_content = String::new();
    open_config.read_to_string(&mut config_content).unwrap();

    let binding = config_content;
    let res1: Vec<&str> = binding.split("\n").collect(); //get line
    let binding2 = res1.clone()[1]; //get line
    let binding3 = binding2.to_string();
    let res2: Vec<&str> = binding3.split("→").collect(); //get left
    let binding4 = res2.clone()[1]; //get left
    let binding5 = binding4.to_string();
    let real_res_tmp = get_url(&binding5); //get http.......

    real_res_tmp.to_string()
}

fn get_url(s: &String) -> &str {
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

fn read_log(log_path: String) {
    let f = File::open(log_path).unwrap();
    let reader = BufReader::new(f);
    for line in reader.lines() {
        // line 是 std::result::Result<std::string::String, std::io::Error> 类型
        // line 不包含换行符
        let line = line.unwrap();
        println!("{}", line);
    }
}

fn use_sqlmap(url: &str, log_path: &str) {
    let tmp_log = "src/module/warning/.log/sql_tmp.log";
    let output = Command::new("sqlmap")
        .arg("-u")
        .arg(url)
        .arg("--batch")
        .output()
        .unwrap();
    let get_res = String::from_utf8(output.stdout).unwrap();
    // println!("{}", get_res);

    // let mut file = fs::File::create(tmp_log);
    let mut file1 = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(tmp_log)
        .unwrap();
    file1.write_all(get_res.as_bytes()).unwrap();

    let mut file2 = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(log_path)
        .unwrap();
    if get_res.contains("[CRITICAL]") {
        file2.write_all(b"").unwrap();
        // println!("hhhhh")
    } else {
        // let pre_to_split_get_res: Vec<&str> = get_res.split("---").collect();
        // let after_split_get_res = pre_to_split_get_res.clone()[1];
        // println!("{}", after_split_get_res);
        let pre_to_split_get_res = Regex::new(r"---").unwrap();
        let after_split_get_res: Vec<&str> =
            pre_to_split_get_res.split(&get_res).into_iter().collect();
        let get_payloads = after_split_get_res.clone()[1];

        let pre_to_split_get_server = Regex::new(r"([\]\[]+)").unwrap();
        let after_split_get_server: Vec<&str> = pre_to_split_get_server
            .split(&after_split_get_res.clone()[2])
            .into_iter()
            .collect();
        let get_server = after_split_get_server.clone()[4];
        // println!("{}", get_server);
        // for split in after_split_get_server {
        //     println!("\"{}\"", split);
        // }

        let final_res = get_payloads.to_string() + "\n" + get_server;

        file2.write_all(final_res.as_bytes()).unwrap();
    }
}
