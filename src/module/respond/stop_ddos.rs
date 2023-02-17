//! @Author       : 白银
//! @Date         : 2023-02-04 16:13:03
//! @LastEditors  : 白银
//! @LastEditTime : 2023-02-17 20:37:40
//! @FilePath     : /rwaf/src/module/respond/stop_ddos.rs
//! @Description  : ddos监测并阻断，可能需要调用反击
//! @Attention    :
//! @Copyright (c) 2023 by 白银 captain-jparrow@qq.com, All Rights Reserved.

use std::{
    fs::{self, File},
    io::{Read, Write},
    process::Command,
    thread,
    time::Duration,
};

use execute::Execute;
use lettre::{transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};
use mysql::{params, prelude::Queryable, Pool};

pub fn stop_ddos_main() {
    loop {
        let _t = thread::spawn(move || stop_ddos());

        thread::sleep(Duration::from_secs(60)); //do every 60s
    }
}

fn stop_ddos() {
    let black_count_max = 2; //设置阈值
    let black_res_init = check_tcp_num();
    let black_res_init_rev = reverse(&black_res_init);
    let black_count: i32 = reverse(&get_count(&black_res_init_rev).to_string().trim())
        .parse()
        .unwrap();
    let black_ip = reverse(&get_ip(&black_res_init_rev).to_string().trim()); //要写入数据库

    // println!("{}", get_date_time());
    let binding = get_date_time();
    let date_time: Vec<&str> = binding.split("\n").collect();
    let now_date = date_time.clone()[0];
    let now_time = date_time.clone()[1];
    let do_what = "prevent ddos";
    let sqlurl = &get_only_sqlurl().to_string()[..];
    // println!("{}", now_time);

    if black_count > black_count_max {
        ban_ip(check_iptables_firewalld(), &black_ip);

        let sender_address = get_only_sender_address(); //like: NoBody <nobody@domain.tld>
        let receiver_address = get_only_receiver_address(); //like: NoBody <nobody@domain.tld>
        let mail_body = "server_time: ".to_string()
            + now_date
            + " "
            + now_time
            + "\nattack source "
            + &black_ip
            + " has been banned"
            + &"\n\n"
            + &"sent by rwaf(https://github.com/Jack-Jparrow/rwaf.git)";
        let email_username = get_only_email_username(); //Email login user name，like: nobody@domain.tld
        let email_passwd = get_only_email_passwd(); //Email login passwd
        let smtp_address = get_only_smtp_address(); //Email smtp address

        let event_id_black: String = now_date.to_string() + now_time + &black_ip;
        let input_event_id_black_respond = super::super::use_sm3::sm3_main(event_id_black);

        write_black_to_sql(
            sqlurl,
            input_event_id_black_respond,
            now_date,
            now_time,
            &black_ip,
        )
        .unwrap();

        send_email(
            sender_address,
            receiver_address,
            mail_body,
            email_username,
            email_passwd,
            &smtp_address,
        );

        let do_res = "true";
        let if_send_email = "true";
        let event_id: String =
            now_date.to_string() + now_time + do_what + &black_ip + do_res + if_send_email;
        let input_event_id = super::super::use_sm3::sm3_main(event_id);

        write_to_respond_log_sql(
            sqlurl,
            input_event_id,
            now_date,
            now_time,
            do_what,
            &black_ip,
            do_res,
            if_send_email,
        )
        .unwrap();
        // write_to_respond_sql(); //需要存放黑名单ip
    } else {
        let do_res = "false";
        let if_send_email = "false";
        let black_ip = "-";
        let event_id: String =
            now_date.to_string() + now_time + do_what + &black_ip + do_res + if_send_email;
        let input_event_id = super::super::use_sm3::sm3_main(event_id);

        write_to_respond_log_sql(
            sqlurl,
            input_event_id,
            now_date,
            now_time,
            do_what,
            black_ip,
            do_res,
            if_send_email,
        )
        .unwrap();

        // write_to_respond_sql(); //黑名单ip为空
    }

    // println!("{}", reverse(&black_res_init));
    // println!("{}", black_count);
    // println!("{}", black_ip);
}

//find ddos source & count
fn check_tcp_num() -> String {
    let mut command = execute::shell(
        "netstat -na|grep ESTABLISHED|awk '{print $5}'|awk -F: '{print $1}'|sort|uniq -c|sort -r",
    );

    let output = command.output().unwrap();
    let connection_state = String::from_utf8(output.stdout).unwrap();

    let connection_state_res: Vec<&str> = connection_state.split("\n").collect();
    // let black_ip = reverse(&get_ip(&black_res_init_rev).to_string().trim());
    let black_res = connection_state_res.clone()[0];
    if black_res.contains("127.0.0.1") {
        let black_res = connection_state_res.clone()[1];
        return black_res.to_string();
    } else {
        let black_res = connection_state_res.clone()[0];
        return black_res.to_string();
    }

    // let mut  open_black_log = File::create("src/module/respond/black.log").unwrap();
    // let mut black_log_content = String::new();
    // open_black_log.write_all(black_res.as_bytes()).unwrap();
    // black_res.to_string()
}

fn check_iptables_firewalld() -> String {
    // let iptables_firewalld = "".to_string();

    let mut if_firewalld = execute::shell("whereis firewalld | awk '{print $2}'");
    // let mut if_iptables = execute::shell("whereis iptabels | awk '{print $2}'",);

    let if_firewalld_res =
        String::from_utf8(if_firewalld.execute_output().unwrap().stdout).unwrap();

    let if_firewalld_path = "src/module/respond/if_firewalld.log";
    let mut f = File::create(if_firewalld_path).unwrap();
    f.write_all(if_firewalld_res.as_bytes()).unwrap();

    let mut open_firewalld = File::open("src/module/respond/if_firewalld.log").unwrap();
    let mut firewalld_content = String::new();
    open_firewalld
        .read_to_string(&mut firewalld_content)
        .unwrap();
    let firewalld_size = fs::metadata("src/module/respond/if_firewalld.log")
        .unwrap()
        .len();

    // let mut open_iptables = File::open("src/module/respond/if_iptables.log").unwrap();
    // let mut iptables_content = String::new();
    // open_iptables.read_to_string(&mut iptables_content).unwrap();
    // let iptables_size = fs::metadata("src/module/respond/if_iptables.log").unwrap().len();

    if firewalld_size > 0 {
        rm_check_iptables_firewalld_log(if_firewalld_path.to_string());

        let iptables_firewalld = "firewalld".to_string();

        return iptables_firewalld;
    } else {
        rm_check_iptables_firewalld_log(if_firewalld_path.to_string());

        let iptables_firewalld = "iptables".to_string();

        return iptables_firewalld;
    }

    // &iptables_firewalld
}

fn ban_ip(which_firewall: String, black_ip: &str) {
    if which_firewall.trim() == "iptables".to_string() {
        let command = "iptables -I INPUT -s ".to_string() + black_ip + &" -j DROP";
        let mut iptables_exec = execute::shell(command);
        let _output = iptables_exec.execute_output().unwrap();
    } else if which_firewall.trim() == "firewalld".to_string() {
        let command = "firewall-cmd --add-rich-rule=\"rule family='ipv4' source address='"
            .to_string()
            + black_ip
            + "' reject\"";
        let mut firewalld_exec = execute::shell(command);
        let _output = firewalld_exec.execute_output().unwrap();
    }
}

fn rm_check_iptables_firewalld_log(log_path: String) {
    let mut command = execute::command_args!("rm", "-rf", log_path);
    let _output = command.execute_output().unwrap();
}

fn send_email(
    sender_address: String,
    receiver_address: String,
    mail_body: String,
    email_username: String,
    email_passwd: String,
    smtp_address: &str,
) {
    let email = Message::builder()
        .from(sender_address.parse().unwrap())
        .to(receiver_address.parse().unwrap())
        .subject("deal with ddos")
        .body(mail_body)
        .unwrap();
    let creds = Credentials::new(email_username, email_passwd);

    let mailer = SmtpTransport::relay(smtp_address)
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("Email prevent ddos sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }
}

fn get_ip(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // return i;
            return &s[0..i];
        }
    }

    // s.len()
    &s[..]
}

fn get_count(s: &String) -> &str {
    let len = s.chars().count();
    let bytes = s.as_bytes();

    // println!("{:?}", bytes.iter().enumerate());

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // println!("{:?}", item);
            // println!("{}", i);
            // println!("{:?}", &s[i+1..i+2]);
            return &s[i + 1..len];
        }
    }

    // s.len()
    &s[..]
}

fn reverse(input: &str) -> String {
    let mut output = String::new();
    for c in input.chars().rev() {
        output.push(c);
    }
    output
}

fn get_date_time() -> String {
    // let mut command = execute::shell("echo $(date +%F%n%T)");
    let output = Command::new("date").arg("+%F%n%T").output().unwrap();
    let res = String::from_utf8(output.stdout).unwrap();

    res
}

fn get_only_sender_address() -> String {
    let mut open_config = File::open("src/config").unwrap();
    let mut config_content = String::new();
    open_config.read_to_string(&mut config_content).unwrap();

    let binding = config_content;
    let res1: Vec<&str> = binding.split("\n").collect(); //get line
    let binding2 = res1.clone()[6]; //get line
    let binding3 = binding2.to_string();
    let res2: Vec<&str> = binding3.split("→").collect(); //get left
    let binding4 = res2.clone()[0]; //get left
    let binding5 = binding4.to_string();
    let real_res_tmp = get_needed_thing(&binding5); //get sender_address

    real_res_tmp.to_string()
}

fn get_only_receiver_address() -> String {
    let mut open_config = File::open("src/config").unwrap();
    let mut config_content = String::new();
    open_config.read_to_string(&mut config_content).unwrap();

    let binding = config_content;
    let res1: Vec<&str> = binding.split("\n").collect(); //get line
    let binding2 = res1.clone()[7]; //get line
    let binding3 = binding2.to_string();
    let res2: Vec<&str> = binding3.split("→").collect(); //get left
    let binding4 = res2.clone()[0]; //get left
    let binding5 = binding4.to_string();
    let real_res_tmp = get_needed_thing(&binding5); //get receiver_address

    real_res_tmp.to_string()
}

fn get_only_email_username() -> String {
    let mut open_config = File::open("src/config").unwrap();
    let mut config_content = String::new();
    open_config.read_to_string(&mut config_content).unwrap();

    let binding = config_content;
    let res1: Vec<&str> = binding.split("\n").collect(); //get line
    let binding2 = res1.clone()[8]; //get line
    let binding3 = binding2.to_string();
    let res2: Vec<&str> = binding3.split("→").collect(); //get left
    let binding4 = res2.clone()[0]; //get left
    let binding5 = binding4.to_string();
    let real_res_tmp = get_needed_thing(&binding5); //get email_username

    real_res_tmp.to_string()
}

fn get_only_email_passwd() -> String {
    let mut open_config = File::open("src/config").unwrap();
    let mut config_content = String::new();
    open_config.read_to_string(&mut config_content).unwrap();

    let binding = config_content;
    let res1: Vec<&str> = binding.split("\n").collect(); //get line
    let binding2 = res1.clone()[9]; //get line
    let binding3 = binding2.to_string();
    let res2: Vec<&str> = binding3.split("→").collect(); //get left
    let binding4 = res2.clone()[0]; //get left
    let binding5 = binding4.to_string();
    let real_res_tmp = get_needed_thing(&binding5); //get email_passwd

    real_res_tmp.to_string()
}

fn get_only_smtp_address() -> String {
    let mut open_config = File::open("src/config").unwrap();
    let mut config_content = String::new();
    open_config.read_to_string(&mut config_content).unwrap();

    let binding = config_content;
    let res1: Vec<&str> = binding.split("\n").collect(); //get line
    let binding2 = res1.clone()[10]; //get line
    let binding3 = binding2.to_string();
    let res2: Vec<&str> = binding3.split("→").collect(); //get left
    let binding4 = res2.clone()[0]; //get left
    let binding5 = binding4.to_string();
    let real_res_tmp = get_needed_thing(&binding5); //get smtp_address

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

struct Payment {
    event_id: String,
    date: String,
    time: String,
    event: String,
    black_ip: String,
    if_banned: String,
    if_send_email: String,
}

fn write_to_respond_log_sql(
    sqlurl: &str,
    input_event_id: String,
    now_date: &str,
    now_time: &str,
    do_what: &str,
    black_ip: &str,
    do_res: &str,
    if_send_email: &str,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let url = sqlurl;

    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;
    let payments = vec![Payment {
        event_id: input_event_id,
        date: now_date.to_string(),
        time: now_time.to_string(),
        event: do_what.to_string(),
        black_ip: black_ip.to_string(),
        if_banned: do_res.to_string(),
        if_send_email: if_send_email.to_string(),
    }];

    conn.exec_batch(
        r"INSERT INTO respond_log (event_id, date, time, event, black_ip, if_banned, if_send_email) VALUES (:event_id, :date, :time, :event, :black_ip, :if_banned, :if_send_email)",
        payments.iter().map(|p| params! {
            "event_id" => p.event_id.clone(),
            "date" => p.date.clone(),
            "time" => &p.time,
            "event" => &p.event,
            "black_ip" => &p.black_ip,
            "if_banned" => &p.if_banned,
            "if_send_email" => &p.if_send_email,
        })
    )?;

    Ok(())
}

struct Payment2 {
    event_id: String,
    date: String,
    time: String,
    black_ip: String,
}

fn write_black_to_sql(
    sqlurl: &str,
    input_event_id_black_respond: String,
    now_date: &str,
    now_time: &str,
    black_ip: &str,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let url = sqlurl;

    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;
    let payments = vec![Payment2 {
        event_id: input_event_id_black_respond,
        date: now_date.to_string(),
        time: now_time.to_string(),
        black_ip: black_ip.to_string(),
    }];

    conn.exec_batch(
        r"INSERT INTO black_ip (event_id, date, time, black_ip) VALUES (:event_id, :date, :time, :black_ip)",
        payments.iter().map(|p| params! {
            "event_id" => p.event_id.clone(),
            "date" => p.date.clone(),
            "time" => &p.time,
            "black_ip" => &p.black_ip,
        })
    )?;

    Ok(())
}
