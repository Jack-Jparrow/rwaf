//! @Author       : 白银
//! @Date         : 2023-02-04 16:13:03
//! @LastEditors  : 白银
//! @LastEditTime : 2023-02-07 18:47:16
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

pub fn stop_ddos_main() {
    loop {
        let t = thread::spawn(move || stop_ddos());

        thread::sleep(Duration::from_secs(20)); //每分钟监测一次
    }
}

fn stop_ddos() {
    let black_count_max = 1; //设置阈值
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
    // println!("{}", now_time);

    if black_count > black_count_max {
        ban_ip(check_iptables_firewalld(), &black_ip);

        let sender_address = "rwaf <jj1017708679@126.com>".to_string(); //like: NoBody <nobody@domain.tld>
        let receiver_address = "Jack Jparrow <captain-jparrow@qq.com>".to_string(); //like: NoBody <nobody@domain.tld>
        let mail_body = "server_time: ".to_string()
            + now_date
            + " "
            + now_time
            + "\nattack source "
            + &black_ip
            + " has been banned"
            + &"\n\n"
            + &"sent by rwaf(https://github.com/Jack-Jparrow/rwaf.git)";
        let email_username = "jj1017708679@126.com".to_string(); //邮箱登陆用户名，like: nobody@domain.tld
        let email_passwd = "H20080808".to_string(); //邮箱登陆密码
        let smtp_address = "smtp.126.com"; //邮箱smtp地址

        send_email(
            sender_address,
            receiver_address,
            mail_body,
            email_username,
            email_passwd,
            smtp_address,
        );
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
    f.write_all(if_firewalld_res.as_bytes());

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
        let output = iptables_exec.execute_output().unwrap();
    } else if which_firewall.trim() == "firewalld".to_string() {
        let command = "firewall-cmd --add-rich-rule=\"rule family='ipv4' source address='"
            .to_string()
            + black_ip
            + "' reject\"";
        let mut firewalld_exec = execute::shell(command);
        let output = firewalld_exec.execute_output().unwrap();
    }
}

fn rm_check_iptables_firewalld_log(log_path: String) {
    let mut command = execute::command_args!("rm", "-rf", log_path);
    let output = command.execute_output().unwrap();
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
        Ok(_) => println!("Email sent successfully!"),
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

fn write_black_to_sql() {
    todo!()
}

fn get_date_time() -> String {
    // let mut command = execute::shell("echo $(date +%F%n%T)");
    let output = Command::new("date").arg("+%F%n%T").output().unwrap();
    let res = String::from_utf8(output.stdout).unwrap();

    res
}
