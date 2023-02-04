//! @Author       : 白银
//! @Date         : 2023-02-04 16:13:03
//! @LastEditors  : 白银
//! @LastEditTime : 2023-02-04 21:57:51
//! @FilePath     : /rwaf/src/module/respond/stop_ddos.rs
//! @Description  : 
//! @Attention    : 
//! @Copyright (c) 2023 by 白银 captain-jparrow@qq.com, All Rights Reserved. 

use lettre::{Message, transport::smtp::authentication::Credentials, SmtpTransport, Transport};

pub fn stop_ddos() {
    let black_res_init = check_tcp_num();
    let black_res_init_rev = reverse(&black_res_init);
    let black_count = reverse(&get_count(&black_res_init_rev).to_string().trim());
    let black_ip = reverse(&get_ip(&black_res_init_rev).to_string().trim());

    // println!("{}", reverse(&black_res_init));
    println!("{}", black_count);
    println!("{}", black_ip);
}

//find ddos source & count
fn check_tcp_num() -> String {
    let mut command = execute::shell(
        "netstat -na|grep ESTABLISHED|awk '{print $5}'|awk -F: '{print $1}'|sort|uniq -c|sort -r",
    );

    let output = command.output().unwrap();
    let connection_state = String::from_utf8(output.stdout).unwrap();

    let connection_state_res: Vec<&str> = connection_state.split("\n").collect();
    let black_res = connection_state_res.clone()[0];

    // let mut  open_black_log = File::create("src/module/respond/black.log").unwrap();
    // let mut black_log_content = String::new();
    // open_black_log.write_all(black_res.as_bytes()).unwrap();
    black_res.to_string()
}

fn ban_ip() {
    todo!()
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
            return &s[i + 1 .. len];
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

fn write_black_to_sql(){
    todo!()
}