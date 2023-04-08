//! @Author       : 白银
//! @Date         : 2023-01-30 21:47:28
//! @LastEditors  : 白银
//! @LastEditTime : 2023-04-08 19:47:33
//! @FilePath     : /rwaf/src/module/protect/show_watch_res.rs
//! @Description  :
//! @Attention    :
//! @Copyright (c) 2023 by 白银 captain-jparrow@qq.com, All Rights Reserved.

use std::{fs::File, io::Read, thread, time::Duration};

use mysql::{params, prelude::Queryable, Pool};

// mod watch;
pub fn show_watch_res_main() {
    loop {
        let _t = thread::spawn(move || {
            let handle_date = thread::spawn(|| super::watch::watch_date::output_get_date_state());
            let handle_time = thread::spawn(|| super::watch::watch_time::output_get_time_state());
            let handle_cpu = thread::spawn(|| super::watch::watch_cpu::output_get_cpu_state());
            let handle_mem =
                thread::spawn(|| super::watch::watch_memory::output_get_mem_state());
            let handle_disk = thread::spawn(|| super::watch::watch_disk::output_get_disk_state());
            let handle_net_receive =
                thread::spawn(|| super::watch::watch_download_net::output_get_net_state_download());
            let handle_net_send =
                thread::spawn(|| super::watch::watch_upload_net::output_get_net_state_upload());

            let date_state = handle_date.join().unwrap();
            let time_state = handle_time.join().unwrap();
            let cpu_state = handle_cpu.join().unwrap();
            let mem_state = handle_mem.join().unwrap();
            let disk_state = handle_disk.join().unwrap();
            let net_state_receive = handle_net_receive.join().unwrap();
            let net_state_send = handle_net_send.join().unwrap();

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
            let event_id: String = date_state.to_string()
                + &time_state
                + do_what
                + &cpu_state
                + &mem_state
                + &disk_state
                + &net_state_receive
                + &net_state_send;
            let input_event_id = super::super::use_sm3::sm3_main(event_id);

            let sqlurl = &get_only_sqlurl().to_string()[..];

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
        });

        thread::sleep(Duration::from_secs(60)); //do every 60s
    }
    // println!("{}", super::super::watch::watch_disk::output_get_disk_state());
    // super::super::watch::watch_cpu::split_from_res_date_time_cpu_state();
}

fn get_only_sqlurl() -> String {
    let mut open_config = File::open("src/config").unwrap();
    let mut config_content = String::new();
    open_config.read_to_string(&mut config_content).unwrap();

    let binding = config_content;
    let res1: Vec<&str> = binding.split("\n").collect(); //get line
    let binding2 = res1.clone()[12]; //get line
    let binding3 = binding2.to_string();
    let res2: Vec<&str> = binding3.split("→").collect(); //get left
    let binding4 = res2.clone()[0]; //get left
    let binding5 = binding4.to_string();
    let real_res_tmp = get_needed_thing(&binding5); //get sqlurl

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
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let url = sqlurl;

    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;
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
