//! @Author       : 白银
//! @Date         : 2023-02-02 16:55:54
//! @LastEditors  : 白银
//! @LastEditTime : 2023-07-03 10:42:09
//! @FilePath     : /rwaf/src/module/detect/check_web_shell.rs
//! @Description  : 监测websehll check webshell
//! @Attention    :
//! @Copyright (c) 2023 by 白银 captain-jparrow@qq.com, All Rights Reserved.

use std::{
    fs::{self, File},
    io::Read,
    path::Path,
    process::Command,
    thread,
    time::Duration,
};

use execute::Execute;
use lettre::{transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};
use mysql::{params, prelude::Queryable, Pool};

// 启动主函数 Main function to start checking for web shells
pub fn start_check_web_shell_main() {
    loop {
        // 使用新线程调用start_check_web_shell函数 Spawn a new thread to call the start_check_web_shell function
        let _t = thread::spawn(move || {
            start_check_web_shell();

            // 获取当前日期和时间 Get the current date and time
            let binding = get_date_time();
            let date_time: Vec<&str> = binding.split("\n").collect();
            let now_date = date_time.clone()[0]; // 获取系统日期 Get system date
            let now_time = date_time.clone()[1]; // 获取系统时间 Get system time

            // 打开dan.log文件并读取内容 Open the dan.log file and read its contents
            let mut open_dan_log = File::open("src/module/detect/dan.log").unwrap();
            let mut dan_log_content = String::new();
            open_dan_log.read_to_string(&mut dan_log_content).unwrap();
            let dan_log_size = fs::metadata("src/module/detect/dan.log").unwrap().len();

            let do_what = "check webshell";
            let sqlurl = &get_only_sqlurl().to_string()[..];

            if dan_log_size > 0 {
                let do_res = "true";
                let if_send_email = "true";
                let event_id: String =
                    now_date.to_string() + now_time + do_what + do_res + if_send_email;
                let input_event_id = super::super::use_sm3::sm3_main(event_id);

                // 将相关数据写入webshell日志和数据库 Write the relevant data to the webshell log and database
                write_to_webshell_log_sql(
                    sqlurl,
                    input_event_id,
                    now_date,
                    now_time,
                    do_what,
                    do_res,
                    if_send_email,
                )
                .unwrap();
            } else {
                let do_res = "false";
                let if_send_email = "false";
                let event_id: String =
                    now_date.to_string() + now_time + do_what + do_res + if_send_email;
                let input_event_id = super::super::use_sm3::sm3_main(event_id);

                // 将相关数据写入webshell日志和数据库 // Write the relevant data to the webshell log and database
                write_to_webshell_log_sql(
                    sqlurl,
                    input_event_id,
                    now_date,
                    now_time,
                    do_what,
                    do_res,
                    if_send_email,
                )
                .unwrap();
            }
        });

        // write_to_webshell_log_sql();

        thread::sleep(Duration::from_secs(3600)); // 每小时一次 Once each hour
    }
}

// 开始检查Webshell Start checking for web shells
fn start_check_web_shell() {
    let dst_path = get_only_dst_path(); // 获取目标路径 Get the target path

    // 定义Python脚本的名称和路径 Define the name and path of the Python script
    let program_name = "python3".to_string();
    let use_script = "src/module/detect/check_web_shell.py".to_string();

    // 如果dan.log文件存在，则删除它 If the dan.log file exists, delete it
    if Path::new("src/module/detect/dan.log").exists() {
        rm_rf_dan_log("src/module/detect/dan.log".to_string());
    }

    check_web_shell(program_name, use_script, dst_path); // 调用check_web_shell函数执行Webshell检查 Call the check_web_shell function to perform web shell detection

    // 打开dan.log文件并读取内容 Open the dan.log file and read its contents
    let mut open_dan_log = File::open("src/module/detect/dan.log").unwrap();
    let mut dan_log_content = String::new();
    open_dan_log.read_to_string(&mut dan_log_content).unwrap();
    let dan_log_size = fs::metadata("src/module/detect/dan.log").unwrap().len();

    // 如果dan.log文件有内容，则发送邮件 If the dan.log file has contents, send an email
    if dan_log_size > 0 {
        // 配置相关的邮件参数 Configure the email parameters
        let sender_address = get_only_sender_address(); // 比如：NoBody <nobody@domain.tld> Like: NoBody <nobody@domain.tld>
        let receiver_address = get_only_receiver_address(); // 比如：NoBody <nobody@domain.tld> Like: NoBody <nobody@domain.tld>
        let mail_body =
            dan_log_content + &"\n\n" + &"sent by rwaf(https://github.com/Jack-Jparrow/rwaf.git)";
        let email_username = get_only_email_username(); // 邮箱用户名，比如：nobody@domain.tld Email login user name，like: nobody@domain.tld
        let email_passwd = get_only_email_passwd(); // 邮箱登陆密码Email login passwd
        let smtp_address = get_only_smtp_address(); // 邮箱smtp地址 Email smtp address

        send_email(
            sender_address,
            receiver_address,
            mail_body,
            email_username,
            email_passwd,
            &smtp_address,
        );
    }
}

// 检查Webshell的函数，接受程序名称、脚本路径和目标路径作为参数 Function to check the web shell, takes program name, script path, and destination path as parameters
fn check_web_shell(program_name: String, use_script: String, dst_path: String) {
    let mut command = execute::command_args!(program_name, use_script, dst_path);
    let _output = command.execute_output().unwrap();
}

// 删除指定路径下的dan.log文件 Function to delete the dan.log file at the specified path
fn rm_rf_dan_log(log_path: String) {
    let mut command = execute::command_args!("rm", "-rf", log_path);
    let _output = command.execute_output().unwrap();
}

// 发送电子邮件函数，接受发件人地址、收件人地址、邮件正文、邮件登录用户名、邮件登录密码和SMTP地址作为参数 Function to send an email, takes sender address, receiver address, mail body, email login username, email login password, and SMTP address as parameters
fn send_email(
    sender_address: String,
    receiver_address: String,
    mail_body: String,
    email_username: String,
    email_passwd: String,
    smtp_address: &str,
) {
    // 配置邮件内容 Configure the email content
    let email = Message::builder()
        .from(sender_address.parse().unwrap())
        .to(receiver_address.parse().unwrap())
        .subject("webshell")
        .body(mail_body)
        .unwrap();
    let creds = Credentials::new(email_username, email_passwd); // 配置邮件传输参数，并发送邮件 Configure the email transport parameters and send the email

    let mailer = SmtpTransport::relay(smtp_address)
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("Email check webshell sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }
}

// 这里的dst_path路径是配置文件的src_path路径 The dst_path here is the src_path of the config file
fn get_only_dst_path() -> String {
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
    let real_res_tmp = get_needed_thing(&binding5); // 获取dst_path Get dst_path

    real_res_tmp.to_string()
}

fn get_only_sender_address() -> String {
    let mut open_config = File::open("src/config").unwrap();
    let mut config_content = String::new();
    open_config.read_to_string(&mut config_content).unwrap();

    let binding = config_content;
    let res1: Vec<&str> = binding.split("\n").collect(); // 获取行 Get line
    let binding2 = res1.clone()[6]; // 获取行 Get line
    let binding3 = binding2.to_string();
    let res2: Vec<&str> = binding3.split("→").collect(); // 获取左侧信息 Get left message
    let binding4 = res2.clone()[0]; // 获取左侧信息 Get left message
    let binding5 = binding4.to_string();
    let real_res_tmp = get_needed_thing(&binding5); // 获取sender_address Get sender_address

    real_res_tmp.to_string()
}

fn get_only_receiver_address() -> String {
    let mut open_config = File::open("src/config").unwrap();
    let mut config_content = String::new();
    open_config.read_to_string(&mut config_content).unwrap();

    let binding = config_content;
    let res1: Vec<&str> = binding.split("\n").collect(); // 获取行 Get line
    let binding2 = res1.clone()[7]; // 获取行 Get line
    let binding3 = binding2.to_string();
    let res2: Vec<&str> = binding3.split("→").collect(); // 获取左侧信息 Get left message
    let binding4 = res2.clone()[0]; // 获取左侧信息 Get left message
    let binding5 = binding4.to_string();
    let real_res_tmp = get_needed_thing(&binding5); // 获取receiver_address Get receiver_address

    real_res_tmp.to_string()
}

fn get_only_email_username() -> String {
    let mut open_config = File::open("src/config").unwrap();
    let mut config_content = String::new();
    open_config.read_to_string(&mut config_content).unwrap();

    let binding = config_content;
    let res1: Vec<&str> = binding.split("\n").collect(); // 获取行 Get line
    let binding2 = res1.clone()[8]; // 获取行 Get line
    let binding3 = binding2.to_string();
    let res2: Vec<&str> = binding3.split("→").collect(); // 获取左侧信息 Get left message
    let binding4 = res2.clone()[0]; // 获取左侧信息 Get left message
    let binding5 = binding4.to_string();
    let real_res_tmp = get_needed_thing(&binding5); // 获取email_username Get email_username

    real_res_tmp.to_string()
}

fn get_only_email_passwd() -> String {
    let mut open_config = File::open("src/config").unwrap();
    let mut config_content = String::new();
    open_config.read_to_string(&mut config_content).unwrap();

    let binding = config_content;
    let res1: Vec<&str> = binding.split("\n").collect(); // 获取行 Get line
    let binding2 = res1.clone()[9]; // 获取行 Get line
    let binding3 = binding2.to_string();
    let res2: Vec<&str> = binding3.split("→").collect(); // 获取左侧信息 Get left message
    let binding4 = res2.clone()[0]; // 获取左侧信息 Get left message
    let binding5 = binding4.to_string();
    let real_res_tmp = get_needed_thing(&binding5); //获取email_passwd Get email_passwd

    real_res_tmp.to_string()
}

fn get_only_smtp_address() -> String {
    let mut open_config = File::open("src/config").unwrap();
    let mut config_content = String::new();
    open_config.read_to_string(&mut config_content).unwrap();

    let binding = config_content;
    let res1: Vec<&str> = binding.split("\n").collect(); // 获取行 Get line
    let binding2 = res1.clone()[10]; // 获取行 Get line
    let binding3 = binding2.to_string();
    let res2: Vec<&str> = binding3.split("→").collect(); // 获取左侧信息 Get left message
    let binding4 = res2.clone()[0]; // 获取左侧信息 Get left message
    let binding5 = binding4.to_string();
    let real_res_tmp = get_needed_thing(&binding5); // 获取smtp_address Get smtp_address

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

fn get_date_time() -> String {
    // let mut command = execute::shell("echo $(date +%F%n%T)");
    let output = Command::new("date").arg("+%F%n%T").output().unwrap(); // 执行带有格式选项的 'date' 命令 Execute the 'date' command with format options
    let res = String::from_utf8(output.stdout).unwrap(); // 将输出的字节数组转换为字符串 Convert the output bytes to a string

    res
}

fn get_only_sqlurl() -> String {
    let mut open_config = File::open("src/config").unwrap(); // 打开配置文件 Open the configuration file
    let mut config_content = String::new();
    open_config.read_to_string(&mut config_content).unwrap(); // 从文件中读取内容 Read the content from the file

    let binding = config_content;
    let res1: Vec<&str> = binding.split("\n").collect(); // 获取行 Get line
    let binding2 = res1.clone()[12]; // 获取行 Get line
    let binding3 = binding2.to_string();
    let res2: Vec<&str> = binding3.split("→").collect(); // 获取左侧信息 Get left message
    let binding4 = res2.clone()[0]; // 获取左侧信息 Get left message
    let binding5 = binding4.to_string();
    let real_res_tmp = get_needed_thing(&binding5); //get sqlurl

    real_res_tmp.to_string()
}

// 定义Payment结构体 Define Payment struct
struct Payment {
    event_id: String,
    date: String,
    time: String,
    event: String,
    if_webshell: String,
    if_send_email: String,
}

fn write_to_webshell_log_sql(
    sqlurl: &str,
    input_event_id: String,
    now_date: &str,
    now_time: &str,
    do_what: &str,
    do_res: &str,
    if_send_email: &str,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let url = sqlurl;

    let pool = Pool::new(url)?; // 创建 SQL 数据库连接池 Create a SQL database connection pool
    let mut conn = pool.get_conn()?; // 从连接池中获取连接 Obtain a connection from the pool
    let payments = vec![Payment {
        // 创建 Payment 结构体对象 Create a Payment struct object
        event_id: input_event_id,        // 输入的事件ID The input event ID
        date: now_date.to_string(),      // 当前日期 Current date
        time: now_time.to_string(),      // 当前时间 Current time
        event: do_what.to_string(),      // 事件描述 Event description
        if_webshell: do_res.to_string(), // 是否为Web Shell标志 Flag for whether it is a web shell
        if_send_email: if_send_email.to_string(), // 发送邮件标志 Flag for sending email
    }];

    conn.exec_batch(
        r"INSERT INTO webshell_log (event_id, date, time, event, if_webshell, if_send_email) VALUES (:event_id, :date, :time, :event, :if_webshell, :if_send_email)",
        payments.iter().map(|p| params! {
            "event_id" => p.event_id.clone(),
            "date" => p.date.clone(),
            "time" => &p.time,
            "event" => &p.event,
            "if_webshell" => &p.if_webshell,
            "if_send_email" => &p.if_send_email,
        })
    )?;

    Ok(())
}
