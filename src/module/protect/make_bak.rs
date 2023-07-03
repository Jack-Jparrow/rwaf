// ! @Author       : 白银
// ! @Date         : 2023-02-01 19:41:02
// ! @LastEditors  : 白银
// ! @LastEditTime : 2023-07-03 09:24:04
// ! @FilePath     : /rwaf/src/module/protect/make_bak.rs
// ! @Description  : 备份文件
// ! @Attention    :
// ! @Copyright (c) 2023 by 白银 captain-jparrow@qq.com, All Rights Reserved.

use std::{fs::File, io::Read, process::Command, thread, time::Duration};

use mysql::{params, prelude::Queryable, Pool};

pub fn use_start_make_bak() {
    loop {
        let _t = thread::spawn(move || start_make_bak()); // 启动备份操作的线程 Start backup operation in a new thread
        let binding = get_date_time();
        let date_time: Vec<&str> = binding.split("\n").collect();

        // 获取当前日期和时间 Get current date and time information
        let now_date = date_time.clone()[0]; // 获取系统日期 Get system date
        let now_time = date_time.clone()[1]; // 获取系统时间 Get system time
        let do_what = "make bak";
        let event_id: String = now_date.to_string() + now_time + do_what;
        let input_event_id = super::super::use_sm3::sm3_main(event_id);
        let sqlurl = &get_only_sqlurl().to_string()[..]; // 获取sqlurl Get sqlurl

        // 将备份操作的相关信息写入日志数据库 Write backup operation information to log database
        write_to_bak_log_sql(sqlurl, input_event_id, now_date, now_time, do_what).unwrap();

        thread::sleep(Duration::from_secs(604800)); // 每周运行一次 Run once a week
    }
}

// 开始备份操作 Start backup operation
pub fn start_make_bak() {
    // 获取源文件路径、目标服务器IP地址、目标服务器用户名和目标路径等配置信息 Get source file path, destination server IP address, destination server username, and destination path from configuration
    let src_path = get_only_src_path();
    let dst_ip = get_only_dst_ip();
    let dst_username = get_only_dst_username();
    let dst_path = get_only_dst_path();

    // 构建执行的shell命令 Build the shell command to execute
    let fin_shell = "scp -r ".to_string()
        + &src_path
        + &" "
        + &dst_username
        + &"@"
        + &dst_ip
        + &":"
        + &dst_path;
    // 执行备份操作 Execute the backup operation
    make_bak(fin_shell);
}

// 执行备份操作的函数 Function to perform backup operation
fn make_bak(fin_shell: String) {
    let mut command = execute::shell(fin_shell);
    let _a = command.output();
}

// 获取配置文件中的src_path Get src_path from configuration file
fn get_only_src_path() -> String {
    let mut open_config = File::open("src/config").unwrap();
    let mut config_content = String::new();
    open_config.read_to_string(&mut config_content).unwrap();

    let binding = config_content;
    let res1: Vec<&str> = binding.split("\n").collect(); // 获取行 Get line
    let binding2 = res1.clone()[2]; // 获取行 Get line
    let binding3 = binding2.to_string();
    let res2: Vec<&str> = binding3.split("→").collect(); // 获取左侧信息 Get left message
    let binding4 = res2.clone()[0]; // 获取左侧信息 Get left message
    let binding5 = binding4.to_string();
    let real_res_tmp = get_needed_thing(&binding5); // 获取src_path Get src_path

    real_res_tmp.to_string()
}

// 获取配置文件中的获取dst_ip Get dst_ip from configuration file
fn get_only_dst_ip() -> String {
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
    let real_res_tmp = get_needed_thing(&binding5); // 获取dst_ip Get dst_ip

    real_res_tmp.to_string()
}

// 获取配置文件中的获取dst_username Get dst_username from configuration file
fn get_only_dst_username() -> String {
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
    let real_res_tmp = get_needed_thing(&binding5); // 获取dst_username Get dst_username

    real_res_tmp.to_string()
}

// 获取配置文件中的dst_path Get dst_path from configuration file
fn get_only_dst_path() -> String {
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
    let real_res_tmp = get_needed_thing(&binding5); // 获取dst_path Get dst_path

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

// 获取日期和时间 Get date and time
fn get_date_time() -> String {
    // let mut command = execute::shell("echo $(date +%F%n%T)");
    let output = Command::new("date").arg("+%F%n%T").output().unwrap(); // 执行date命令获取日期和时间，并返回结果字符串 Execute the "date" command to get the date and time, and return the result as a string
    let res = String::from_utf8(output.stdout).unwrap();

    res
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
    let real_res_tmp = get_needed_thing(&binding5); // 获取sqlurl Get sqlurl

    real_res_tmp.to_string()
}

// 定义Payment结构体 Define Payment struct
struct Payment {
    event_id: String,
    date: String,
    time: String,
    event: String,
}

// 将日志写入数据库 Write logs to the database
fn write_to_bak_log_sql(
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
        r"INSERT INTO backup_log (event_id, date, time, event) VALUES (:event_id, :date, :time, :event)",
        payments.iter().map(|p| params! {
            "event_id" => p.event_id.clone(),
            "date" => p.date.clone(),
            "time" => &p.time,
            "event" => &p.event,
        })
    )?;

    Ok(())
}
