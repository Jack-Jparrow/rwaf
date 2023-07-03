//! @Author       : 白银
//! @Date         : 2023-01-30 21:47:28
//! @LastEditors  : 白银
//! @LastEditTime : 2023-04-08 19:47:33
//! @FilePath     : /rwaf/src/module/protect/show_watch_res.rs
//! @Description  : 展示系统性能监测结果 Display system performance monitoring results
//! @Attention    :
//! @Copyright (c) 2023 by 白银 captain-jparrow@qq.com, All Rights Reserved.

use std::{fs::File, io::Read, thread, time::Duration};

use mysql::{params, prelude::Queryable, Pool};

// 定义show_watch_res_main函数作为主函数入口 Define the show_watch_res_main function as the main entry point
pub fn show_watch_res_main() {
    loop {
        // 创建一个新线程 Create a new thread
        let _t = thread::spawn(move || {
            let handle_date = thread::spawn(|| super::watch::watch_date::output_get_date_state()); // 在新线程中获取日期状态 Get date state in a new thread
            let handle_time = thread::spawn(|| super::watch::watch_time::output_get_time_state()); // 在新线程中获取时间状态 Get time state in a new thread
            let handle_cpu = thread::spawn(|| super::watch::watch_cpu::output_get_cpu_state());  // 在新线程中获取CPU状态 Get CPU state in a new thread
            let handle_mem =
                thread::spawn(|| super::watch::watch_memory::output_get_mem_state()); // 在新线程中获取内存状态 Get memory state in a new thread
            let handle_disk = thread::spawn(|| super::watch::watch_disk::output_get_disk_state()); // 在新线程中获取磁盘状态 Get disk state in a new thread
            let handle_net_receive =
                thread::spawn(|| super::watch::watch_download_net::output_get_net_state_download()); // 在新线程中获取网络接收状态 Get network receive state in a new thread
            let handle_net_send =
                thread::spawn(|| super::watch::watch_upload_net::output_get_net_state_upload()); // 在新线程中获取网络发送状态 Get network send state in a new thread

            let date_state = handle_date.join().unwrap(); // 等待日期状态线程结束并取得结果 Wait for the date state thread to finish and get the result
            let time_state = handle_time.join().unwrap(); // 等待时间状态线程结束并取得结果 Wait for the time state thread to finish and get the result
            let cpu_state = handle_cpu.join().unwrap(); // 等待CPU状态线程结束并取得结果 Wait for the CPU state thread to finish and get the result
            let mem_state = handle_mem.join().unwrap(); // 等待内存状态线程结束并取得结果  Wait for the memory state thread to finish and get the result
            let disk_state = handle_disk.join().unwrap(); // 等待磁盘状态线程结束并取得结果 Wait for the disk state thread to finish and get the result
            let net_state_receive = handle_net_receive.join().unwrap(); // 等待网络接收状态线程结束并取得结果 Wait for the network receive state thread to finish and get the result
            let net_state_send = handle_net_send.join().unwrap(); // 等待网络发送状态线程结束并取得结果 Wait for the network send state thread to finish and get the result

            println!(
                "{} {} {} {} {} {} {}",
                date_state,
                time_state,
                cpu_state,
                mem_state,
                disk_state,
                net_state_receive,
                net_state_send
            );

            let do_what = "system_monitor";
            // 根据获取到的状态拼接成事件ID明文 Concatenate the obtained states into a plaintext event ID
            let event_id: String = date_state.to_string()
                + &time_state
                + do_what
                + &cpu_state
                + &mem_state
                + &disk_state
                + &net_state_receive
                + &net_state_send;
            let input_event_id = super::super::use_sm3::sm3_main(event_id); // 对事件ID进行加密得到事件ID密文，即真正写进数据库的ID Encrypt the event ID to obtain the ciphertext ID that is actually written to the database

            let sqlurl = &get_only_sqlurl().to_string()[..]; // 获取数据库连接URL  Get the database connection URL

            // 将监控状态写入数据库 Write the monitoring state to the database
            write_to_system_monitor_sql(
                sqlurl,
                input_event_id,
                &date_state,
                &time_state,
                do_what,
                &cpu_state,
                &mem_state,
                &disk_state,
                &net_state_receive,
                &net_state_send,
            )
            .unwrap();
        }); // 线程结束 Thread ends

        thread::sleep(Duration::from_secs(60)); // 每分钟一次 Once per minute
    }
    // println!("{}", super::super::watch::watch_disk::output_get_disk_state());
    // super::super::watch::watch_cpu::split_from_res_date_time_cpu_state();
}

// 获取数据库连接URL Get the database connection URL
fn get_only_sqlurl() -> String {
    let mut open_config = File::open("src/config").unwrap(); // 打开配置文件 Open the configuration file
    let mut config_content = String::new();
    open_config.read_to_string(&mut config_content).unwrap(); // 读取配置文件内容 Read the contents of the configuration file

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

// 定义Payment结构体 Define the Payment struct
struct Payment {
    event_id: String,
    date: String,
    time: String,
    event: String,
    cpu: String,
    mem: String,
    disk: String,
    net_receive: String,
    net_send: String,
}

fn write_to_system_monitor_sql(
    sqlurl: &str,
    input_event_id: String,
    now_date: &str,
    now_time: &str,
    do_what: &str,
    cpu_state: &String,
    mem_state: &String,
    disk_state: &String,
    net_state_receive: &String,
    net_state_send: &String,
) -> std::result::Result<(), Box<dyn std::error::Error>> {  // 将监控数据写入数据库 Write monitoring data to the database
    let url = sqlurl; // 获取数据库连接URL Get the database connection URL

    let pool = Pool::new(url)?; // 创建数据库连接池 Create a database connection pool
    let mut conn = pool.get_conn()?; // 获取数据库连接 Get database connection
    // 定义Payment结构体的实例 Define an instance of the Payment struct
    let payments = vec![Payment {
        event_id: input_event_id,
        date: now_date.to_string(),
        time: now_time.to_string(),
        event: do_what.to_string(),
        cpu: cpu_state.to_string(),
        mem: mem_state.to_string(),
        disk: disk_state.to_string(),
        net_receive: net_state_receive.to_string(),
        net_send: net_state_send.to_string(),
    }];

    // 执行SQL插入语句将监控数据写入数据库 Execute the SQL insert statement to write monitoring data to the database
    conn.exec_batch(
        r"INSERT INTO system_monitor (event_id, date, time, event, cpu, mem, disk, net_receive, net_send) VALUES (:event_id, :date, :time, :event, :cpu, :mem, :disk, :net_receive, :net_send)",
        payments.iter().map(|p| params! {
            "event_id" => p.event_id.clone(),
            "date" => p.date.clone(),
            "time" => &p.time,
            "event" => &p.event,
            "cpu" => &p.cpu,
            "mem" => &p.mem,
            "disk" => &p.disk,
            "net_receive" => &p.net_receive,
            "net_send" => &p.net_send,
        })
    )?;

    Ok(())
}
