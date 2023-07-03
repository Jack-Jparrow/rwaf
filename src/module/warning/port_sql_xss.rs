//! @Author       : 白银
//! @Date         : 2023-02-12 18:56:47
//! @LastEditors  : 白银
//! @LastEditTime : 2023-07-03 10:10:26
//! @FilePath     : /rwaf/src/module/warning/port_sql_xss.rs
//! @Description  : 一次性任务，运行检查 One time task, running inspection
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
    let url = get_only_url(); // 获取URL // Get the URL

    let port_scan_script = "src/module/warning/port_scan.py".to_string(); // 端口扫描脚本路径 Path to the port scan script
    let xss_script = "src/module/warning/xss.py".to_string(); // XSS测试脚本路径 Path to the XSS test script

    start_rm(); // 调用start_rm函数，删除之前的日志文件 Call the start_rm function to delete previous log files

    println!("......Port scan is working, please wait......");
    port_scan(port_scan_script); // 执行端口扫描任务 Execute the port scan task
    let port_scan_script_log = "src/module/warning/.log/port.log".to_string();
    let port_scan_script_log_size = fs::metadata("src/module/warning/.log/port.log")
        .unwrap()
        .len();
    println!("***Port scan is done***\n");
    if port_scan_script_log_size > 0 {
        read_log(port_scan_script_log); // 读取并输出端口扫描日志文件内容 Read and output the contents of the port scan log file

        println!("!!!Please confirm whether some ports are in use!!!\n")
    }
    println!("------------------------------------------------------------------------------\n");

    println!("......Sql injection test is working, please wait......");
    // check_sql_injection(sql_script, &url);
    let sql_script_log = "src/module/warning/.log/sql.log".to_string();
    use_sqlmap(&url, &sql_script_log); // 使用sqlmap工具进行SQL注入测试 Use sqlmap tool to perform SQL injection testing

    let sql_script_log_size = fs::metadata("src/module/warning/.log/sql.log")
        .unwrap()
        .len();
    println!("***Sql injection test is done***");
    if sql_script_log_size > 0 {
        read_log(sql_script_log); // 读取并输出SQL注入日志文件内容 Read and output the contents of the SQL injection log file

        println!("!!!There is a risk of sql injection!!!\n")
    } else {
        println!("@@@There is no risk of sql injection@@@\n")
    }
    println!("------------------------------------------------------------------------------\n");

    println!("......Xss test is working, please wait......");
    check_xss(xss_script, &url); // 执行XSS测试任务 Execute the XSS test task
    let xss_script_log = "src/module/warning/.log/xss.log".to_string();
    let xss_script_log_size = fs::metadata("src/module/warning/.log/xss.log")
        .unwrap()
        .len();
    println!("***Xss test is done***");
    if xss_script_log_size > 0 {
        read_log(xss_script_log); // 读取并输出XSS日志文件内容 Read and output the contents of the XSS log file

        println!("!!!There is a risk of xss!!!\n")
    }
    println!("------------------------------------------------------------------------------");
}

fn start_rm() {
    // delete port_scan log
    if Path::new("src/module/warning/.log/port.log").exists() {
        rm_rf_log("src/module/warning/.log/port.log".to_string()); // 删除端口扫描日志文件 Delete the port scan log file
    }

    // delete sql log
    if Path::new("src/module/warning/.log/sql.log").exists() {
        rm_rf_log("src/module/warning/.log/sql.log".to_string()); // 删除SQL注入日志文件 Delete the SQL injection log file
    }

    // delete sql_tmp log
    if Path::new("src/module/warning/.log/sql_tmp.log").exists() {
        rm_rf_log("src/module/warning/.log/sql_tmp.log".to_string()); // 删除SQL临时日志文件 Delete the SQL temporary log file
    }

    // delete xss log
    if Path::new("src/module/warning/.log/xss.log").exists() {
        rm_rf_log("src/module/warning/.log/xss.log".to_string()); // 删除XSS日志文件 Delete the XSS log file
    }
}

fn rm_rf_log(log_path: String) {
    let mut command = execute::command_args!("rm", "-rf", log_path);
    let _output = command.execute_output().unwrap(); // 执行删除文件命令 Execute the delete file command
}

fn port_scan(port_scan_script: String) {
    let mut command = execute::command_args!("python3".to_string(), port_scan_script);
    let _output = command.execute_output().unwrap(); // 执行端口扫描脚本 Execute the port scan script
}

fn check_xss(xss_script: String, url: &str) {
    let mut command = execute::command_args!("python3".to_string(), xss_script, url.to_string());
    let _output = command.execute_output().unwrap(); // 执行XSS测试脚本 Execute the XSS test script
}

fn get_only_url() -> String {
    let mut open_config = File::open("src/config").unwrap(); // 打开配置文件 Open the configuration file
    let mut config_content = String::new();
    open_config.read_to_string(&mut config_content).unwrap(); // 读取配置文件内容 Read the content of the configuration file

    let binding = config_content; // 配置文件内容赋值给binding字符串 Assign the content of the configuration file to the "binding" string
    let res1: Vec<&str> = binding.split("\n").collect(); // 获取行 Get line
    let binding2 = res1.clone()[1]; // 获取行 Get line
    let binding3 = binding2.to_string();
    let res2: Vec<&str> = binding3.split("→").collect(); // 获取左侧信息 Get left message
    let binding4 = res2.clone()[0]; // 获取左侧信息 Get left message
    let binding5 = binding4.to_string();
    let real_res_tmp = get_url(&binding5); // 获取http…… Get http.......

    real_res_tmp.to_string()
}

// 获取目标地址 Get the target address
fn get_url(s: &String) -> &str {
    let len = s.trim().chars().count();
    let bytes = s.as_bytes();

    // 遍历字符串，寻找“=”的位置，提取“=”后面的部分作为结果 Iterate through the string, find the position of "=", and extract the part after "=" as the result
    for (i, &item) in bytes.iter().enumerate() {
        if item == b'=' {
            return &s[i + 1..len];
        }
    }

    // s.len()
    &s[..] // 如果没有等号，返回整个字符串 If there is no "=", return the entire string
}

fn read_log(log_path: String) {
    let f = File::open(log_path).unwrap(); // 打开日志文件 Open the log file
    let reader = BufReader::new(f);
    for line in reader.lines() {
        // line 是 std::result::Result<std::string::String, std::io::Error> 类型
        // line 不包含换行符
        let line = line.unwrap(); // 逐行读取日志文件内容并输出到控制台 Read the content of the log file line by line and output to the console
        println!("{}", line);
    }
}

fn use_sqlmap(url: &str, log_path: &str) {
    let tmp_log = "src/module/warning/.log/sql_tmp.log"; // SQL临时日志文件路径 SQL temporary log file path
                                                         // 使用sqlmap工具进行SQL注入测试
    let output = Command::new("sqlmap")
        .arg("-u")
        .arg(url)
        .arg("--batch")
        .output()
        .unwrap();
    let get_res = String::from_utf8(output.stdout).unwrap(); // 获取sqlmap输出的结果 Get the result output by sqlmap
                                                             // println!("{}", get_res);

    // 创建或打开SQL临时日志文件
    let mut file1 = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(tmp_log)
        .unwrap();
    file1.write_all(get_res.as_bytes()).unwrap(); // 将sqlmap结果写入SQL临时日志文件 Write sqlmap result to the SQL temporary log file

    // 创建或打开SQL注入日志文件 Create or open the SQL injection log file
    let mut file2 = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(log_path)
        .unwrap();
    // 如果输出包含“[CRITICAL]”，则清空SQL注入日志文件 If the output contains "[CRITICAL]", clear the SQL injection log file
    if get_res.contains("[CRITICAL]") {
        file2.write_all(b"").unwrap();
        // println!("hhhhh")
    } else {
        // let pre_to_split_get_res: Vec<&str> = get_res.split("---").collect();
        // let after_split_get_res = pre_to_split_get_res.clone()[1];
        // println!("{}", after_split_get_res);
        let pre_to_split_get_res = Regex::new(r"---").unwrap();
        let after_split_get_res: Vec<&str> =
            pre_to_split_get_res.split(&get_res).into_iter().collect(); // 使用正则表达式分割sqlmap结果 Split the sqlmap result using regular expression
        let get_payloads = after_split_get_res.clone()[1]; // 获取分割后的结果的第二部分并去除首尾空格 Get the second part of the split result and remove leading/trailing whitespaces

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

        file2.write_all(final_res.as_bytes()).unwrap(); // 将结果写入SQL注入日志文件 Write the result to the SQL injection log file
    }
}
