//! @Author       : 白银
//! @Date         : 2023-02-02 16:55:54
//! @LastEditors  : 白银
//! @LastEditTime : 2023-02-15 19:32:38
//! @FilePath     : /rwaf/src/module/detect/check_web_shell.rs
//! @Description  :
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

pub fn start_check_web_shell_main() {
    loop {
        let t = thread::spawn(move || {
            start_check_web_shell();

            let binding = get_date_time();
            let date_time: Vec<&str> = binding.split("\n").collect();
            let now_date = date_time.clone()[0]; //get system date
            let now_time = date_time.clone()[1]; //get system time

            let mut open_dan_log = File::open("src/module/detect/dan.log").unwrap();
            let mut dan_log_content = String::new();
            open_dan_log.read_to_string(&mut dan_log_content).unwrap();
            let dan_log_size = fs::metadata("src/module/detect/dan.log").unwrap().len();

            let do_what = "check webshell";

            if dan_log_size > 0 {
                let do_res = true;
                let if_send_email = true;

                write_to_webshell_log_sql();
            } else {
                let do_res = false;
                let if_send_email = false;

                write_to_webshell_log_sql();
            }
        });

        // write_to_webshell_log_sql();

        thread::sleep(Duration::from_secs(5184000)); //do every 24h
    }
}

fn start_check_web_shell() {
    let dst_path = get_only_dst_path();
    // let src_ip = "129.226.211.132";
    // let src_username = "root";
    // let src_path = "/home/lighthouse/pwn";

    let program_name = "python3".to_string();
    let use_script = "src/module/detect/check_web_shell.py".to_string();

    if Path::new("src/module/detect/dan.log").exists() {
        rm_rf_dan_log("src/module/detect/dan.log".to_string());
    }

    check_web_shell(program_name, use_script, dst_path);

    let mut open_dan_log = File::open("src/module/detect/dan.log").unwrap();
    let mut dan_log_content = String::new();
    open_dan_log.read_to_string(&mut dan_log_content).unwrap();
    let dan_log_size = fs::metadata("src/module/detect/dan.log").unwrap().len();

    if dan_log_size > 0 {
        let sender_address = get_only_sender_address(); //like: NoBody <nobody@domain.tld>
        let receiver_address = get_only_receiver_address(); //like: NoBody <nobody@domain.tld>
        let mail_body =
            dan_log_content + &"\n\n" + &"sent by rwaf(https://github.com/Jack-Jparrow/rwaf.git)";
        let email_username = get_only_email_username(); //Email login user name，like: nobody@domain.tld
        let email_passwd = get_only_email_passwd(); //Email login passwd
        let smtp_address = get_only_smtp_address(); //Email smtp address

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

fn check_web_shell(program_name: String, use_script: String, dst_path: String) {
    let mut command = execute::command_args!(program_name, use_script, dst_path);
    let output = command.execute_output().unwrap();
}

fn rm_rf_dan_log(log_path: String) {
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
        .subject("webshell")
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

//The dst_path here is the src_path of the config file
fn get_only_dst_path() -> String {
    let mut open_config = File::open("src/config").unwrap();
    let mut config_content = String::new();
    open_config.read_to_string(&mut config_content).unwrap();

    let binding = config_content;
    let res1: Vec<&str> = binding.split("\n").collect(); //get line
    let binding2 = res1.clone()[2]; //get line
    let binding3 = binding2.to_string();
    let res2: Vec<&str> = binding3.split("→").collect(); //get left
    let binding4 = res2.clone()[0]; //get left
    let binding5 = binding4.to_string();
    let real_res_tmp = get_needed_thing(&binding5); //get dst_path

    real_res_tmp.to_string()
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

fn get_date_time() -> String {
    // let mut command = execute::shell("echo $(date +%F%n%T)");
    let output = Command::new("date").arg("+%F%n%T").output().unwrap();
    let res = String::from_utf8(output.stdout).unwrap();

    res
}

fn write_to_webshell_log_sql() {
    todo!()
}
