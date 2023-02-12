//! @Author       : 白银
//! @Date         : 2023-02-12 18:56:47
//! @LastEditors  : 白银
//! @LastEditTime : 2023-02-12 20:29:54
//! @FilePath     : /rwaf/src/module/warning/port_sql_xss.rs
//! @Description  : 
//! @Attention    : 
//! @Copyright (c) 2023 by 白银 captain-jparrow@qq.com, All Rights Reserved. 

use std::{path::Path, fs::{File, self}, io::{Read, BufReader, BufRead}};
use execute::Execute;

pub fn port_sql_xss_main(){

    let url = get_only_url();
    
    let port_scan_script = "src/module/warning/port_scan.py".to_string();
    let sql_script = "src/module/warning/sql.py".to_string();
    let xss_script = "src/module/warning/xss.py".to_string();

    start_rm();

    println!("......Port scan is working, please wait......");
    port_scan(port_scan_script);
    let port_scan_script_log = "src/module/warning/.log/port.log".to_string();
    let port_scan_script_log_size = fs::metadata("src/module/warning/.log/port.log").unwrap().len();
    println!("***Port scan is done***");
    if port_scan_script_log_size > 0 {
        read_log(port_scan_script_log);

        println!("!!!Please confirm whether some ports are in use!!!")
    }
    println!("------------------------------------------------------------------------------\n");

    println!("......Sql injection test is working, please wait......");
    check_sql_injection(sql_script, &url);
    let sql_script_log = "src/module/warning/.log/sql.log".to_string();
    let sql_script_log_size = fs::metadata("src/module/warning/.log/sql.log").unwrap().len();
    println!("***Sql injection test is done***");
    if sql_script_log_size > 0 {
        read_log(sql_script_log);

        println!("!!!There is a risk of sql injection!!!")
    }
    println!("------------------------------------------------------------------------------\n");

    println!("......Xss test is working, please wait......");
    check_xss(xss_script, &url);
    let xss_script_log = "src/module/warning/.log/xss.log".to_string();
    let xss_script_log_size = fs::metadata("src/module/warning/.log/xss.log").unwrap().len();
    println!("***Xss test is done***");
    if xss_script_log_size > 0 {
        read_log(xss_script_log);

        println!("!!!There is a risk of xss!!!")
    }
    println!("------------------------------------------------------------------------------");

}

fn start_rm(){

    //delete port_scan log
    if Path::new("src/module/warning/.log/port.log").exists() {
        rm_rf_log("src/module/warning/.log/port.log".to_string());
    }
    
    //delete sql log
    if Path::new("src/module/warning/.log/sql.log").exists() {
        rm_rf_log("src/module/warning/.log/sql.log".to_string());
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

fn check_sql_injection(sql_script: String, url: &str) {
    let mut command = execute::command_args!("python3".to_string(), sql_script, url.to_string());
    let _output = command.execute_output().unwrap();
}

fn check_xss(xss_script: String, url: &str) {
    let mut command = execute::command_args!("python3".to_string(), xss_script, url.to_string());
    let _output = command.execute_output().unwrap();
}

fn get_only_url() -> String{
    let mut open_config = File::open("src/config").unwrap();
    let mut config_content = String::new();
    open_config.read_to_string(&mut config_content).unwrap();

    let binding = config_content;
    let res1: Vec<&str> = binding.split("\n").collect();//get line
    // println!("{:?}", res1);
    let binding2 = res1.clone()[0];//get line
    let binding3 = binding2.to_string();
    let res2: Vec<&str> = binding3.split("→").collect();//get left
    let binding4 = res2.clone()[0];//get left
    let binding5 = binding4.to_string();
    let real_res_tmp =get_url(&binding5);//get http.......
    // binding4.to_string()
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

fn read_log(log_path: String){
    let f = File::open(log_path).unwrap();
    let reader = BufReader::new(f);
    for line in reader.lines() {
        // line 是 std::result::Result<std::string::String, std::io::Error> 类型
        // line 不包含换行符
        let line = line.unwrap();
        println!("{}", line);
    }
}