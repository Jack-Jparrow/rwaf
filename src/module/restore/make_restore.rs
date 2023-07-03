//! @Author       : 白银
//! @Date         : 2023-02-01 20:23:32
//! @LastEditors  : 白银
//! @LastEditTime : 2023-07-03 09:13:04
//! @FilePath     : /rwaf/src/module/restore/make_restore.rs
//! @Description  : 从备份服务器恢复 Restore from bak_server
//! @Attention    :
//! @Copyright (c) 2023 by 白银 captain-jparrow@qq.com, All Rights Reserved.

use std::{fs::File, io::Read, process::Command};

use mysql::{params, prelude::Queryable, Pool};

pub fn start_make_restore() {
    let dst_path = get_only_dst_path(); // 获取目标路径 Get dst_path
    let src_ip = get_only_src_ip(); // 获取源服务器IP地址 Get src_ip
    let src_username = get_only_src_username(); // 获取源服务器用户名 Get src_username
    let src_path = get_only_src_path(); // 获取源路径 Get src_path

    // 构造完整的shell命令 Construct the complete shell command
    let fin_shell = "scp -r ".to_string()
        + &src_username
        + &"@"
        + &src_ip
        + &":"
        + &src_path
        + &" "
        + &dst_path;
    // println!("start {}", &fin_shell);

    // println!("{}", make_restore(fin_shell));
    let binding = get_date_time();
    let date_time: Vec<&str> = binding.split("\n").collect();
    let now_date = date_time.clone()[0]; // 获取系统日期 Get system date
    let now_time = date_time.clone()[1]; // 获取系统时间 Get system time
    let sqlurl = &get_only_sqlurl().to_string()[..]; // 获取sqlurl Get sqlurl

    make_restore(fin_shell); // 调用make_restore函数执行shell命令 Call the make_restore function to execute the shell command

    // 生成事件ID Generate event ID
    let do_what = "make restore";
    let event_id: String = now_date.to_string() + now_time + do_what;
    let input_event_id = super::super::use_sm3::sm3_main(event_id); // sm3加密结果即为ID The result of sm3 encryption is the ID

    write_to_respond_log_sql(sqlurl, input_event_id, now_date, now_time, do_what).unwrap();
    // 将日志写入数据库 Write logs to the database
}

fn make_restore(fin_shell: String) {
    // println!("{}", &fin_shell);
    let mut command = execute::shell(fin_shell); // 执行shell命令 Execute the shell command
                                                 // let output = Command::new("echo").args(["123".to_string()]).output().unwrap();

    // 检查命令执行状态 Check the execution status of the command
    match command.status().unwrap().code() {
        Some(code) => {
            println!("Exit Status: {}", code);
        }
        None => {
            println!("Process terminated");
        }
    }
    // println!("done");
    // let res = String::from_utf8(output.stdout).unwrap();

    // res.trim().to_string()
}

// 获取日期和时间 Get date and time
fn get_date_time() -> String {
    // let mut command = execute::shell("echo $(date +%F%n%T)");
    let output = Command::new("date").arg("+%F%n%T").output().unwrap(); // 执行date命令获取日期和时间，并返回结果字符串 Execute the "date" command to get the date and time, and return the result as a string
    let res = String::from_utf8(output.stdout).unwrap();

    res
}

// 这里的dst_path是配置文件中的another_src_path The dst_path here is the another_src_path of the config file
fn get_only_dst_path() -> String {
    let mut open_config = File::open("src/config").unwrap();
    let mut config_content = String::new();
    open_config.read_to_string(&mut config_content).unwrap();

    let binding = config_content;
    let res1: Vec<&str> = binding.split("\n").collect(); // 获取行 Get line
    let binding2 = res1.clone()[11]; // 获取行 Get line
    let binding3 = binding2.to_string();
    let res2: Vec<&str> = binding3.split("→").collect(); // 获取左侧信息 Get left message
    let binding4 = res2.clone()[0]; // 获取左侧信息 Get left message
    let binding5 = binding4.to_string();
    let real_res_tmp = get_needed_thing(&binding5); // 获取dst_path Get dst_path

    // println!("{}", &real_res_tmp);
    real_res_tmp.to_string()
}

// 这里的src_ip是配置文件的dst_ip The src_ip here is the dst_ip of the config file
fn get_only_src_ip() -> String {
    let mut open_config = File::open("src/config").unwrap();
    let mut config_content = String::new();
    open_config.read_to_string(&mut config_content).unwrap();

    let binding = config_content;
    let res1: Vec<&str> = binding.split("\n").collect(); // 获取行 Get line
    let binding2 = res1.clone()[3]; // 获取行 Get line
    let binding3 = binding2.to_string();
    let res2: Vec<&str> = binding3.split("→").collect(); // 获取左侧信息 Get left message
    let binding4 = res2.clone()[0]; // 获取左侧信息 Get left message
    let binding5 = binding4.to_string();
    let real_res_tmp = get_needed_thing(&binding5); // 获取src_ip Get src_ip

    real_res_tmp.to_string()
}

// 这里的src_username是配置文件的dst_username The src_username here is the dst_username of the config file
fn get_only_src_username() -> String {
    let mut open_config = File::open("src/config").unwrap();
    let mut config_content = String::new();
    open_config.read_to_string(&mut config_content).unwrap();

    let binding = config_content;
    let res1: Vec<&str> = binding.split("\n").collect(); // 获取行 Get line
    let binding2 = res1.clone()[4]; // 获取行 Get line
    let binding3 = binding2.to_string();
    let res2: Vec<&str> = binding3.split("→").collect(); // 获取左侧信息 Get left message
    let binding4 = res2.clone()[0]; // 获取左侧信息 Get left message
    let binding5 = binding4.to_string();
    let real_res_tmp = get_needed_thing(&binding5); // 获取src_username Get src_username

    real_res_tmp.to_string()
}

// 这里的src_path路径是配置文件的dst_path路径 The src_path here is the dst_path of the config file
fn get_only_src_path() -> String {
    let mut open_config = File::open("src/config").unwrap();
    let mut config_content = String::new();
    open_config.read_to_string(&mut config_content).unwrap();

    let binding = config_content;
    let res1: Vec<&str> = binding.split("\n").collect(); // 获取行 Get line
    let binding2 = res1.clone()[5]; // 获取行 Get line
    let binding3 = binding2.to_string();
    let res2: Vec<&str> = binding3.split("→").collect(); // 获取左侧信息 Get left message
    let binding4 = res2.clone()[0]; // 获取左侧信息 Get left message
    let binding5 = binding4.to_string();
    let real_res_tmp = get_needed_thing(&binding5); // 获取src_path Get src_path

    // println!("{}", &real_res_tmp);
    real_res_tmp.to_string()
}

// 获取数据库URL Get sqlurl
fn get_only_sqlurl() -> String {
    let mut open_config = File::open("src/config").unwrap();
    let mut config_content = String::new();
    open_config.read_to_string(&mut config_content).unwrap();

    let binding = config_content;
    let res1: Vec<&str> = binding.split("\n").collect(); // 获取行 Get line
    let binding2 = res1.clone()[12]; // 获取行 Get line
    let binding3 = binding2.to_string();
    let res2: Vec<&str> = binding3.split("→").collect(); // 获取左侧信息 Get left message
    let binding4 = res2.clone()[0]; // 获取左侧信息 Get left message
    let binding5 = binding4.to_string();
    let real_res_tmp = get_needed_thing(&binding5); // 获取数据库URL Get sqlurl

    real_res_tmp.to_string()
}

// 从字符串中提取所需内容 Extract the required content from the string
fn get_needed_thing(s: &String) -> &str {
    let len = s.trim().chars().count(); // 获取字符串修剪后的长度 Get the length of the string after trimming whitespace
    let bytes = s.as_bytes(); // 将字符串转换为字节数组 Convert the string to bytes

    for (i, &item) in bytes.iter().enumerate() {
        if item == b'=' {
            return &s[i + 1..len]; // 返回'='字符后的子字符串 Return the substring after the '=' character
        }
    }

    // s.len()
    &s[..] // 返回所需内容 Return the required content
}

// 定义Payment结构体 Define Payment struct
struct Payment {
    event_id: String,
    date: String,
    time: String,
    event: String,
}

// 将日志写入数据库 Write logs to the database
fn write_to_respond_log_sql(
    sqlurl: &str,
    input_event_id: String,
    now_date: &str,
    now_time: &str,
    do_what: &str,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let url = sqlurl;

    // 建立数据库连接池 Establish database connection pool
    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;
    let payments = vec![Payment {
        event_id: input_event_id,
        date: now_date.to_string(),
        time: now_time.to_string(),
        event: do_what.to_string(),
    }];

    // 批量插入数据 Batch insert data
    conn.exec_batch(
        r"INSERT INTO respond_log (event_id, date, time, event) VALUES (:event_id, :date, :time, :event)",
        payments.iter().map(|p| params! {
            "event_id" => p.event_id.clone(),
            "date" => p.date.clone(),
            "time" => &p.time,
            "event" => &p.event,
        })
    )?;

    Ok(())
}
