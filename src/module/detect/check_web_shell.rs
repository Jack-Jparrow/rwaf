//! @Author       : 白银
//! @Date         : 2023-02-02 16:55:54
//! @LastEditors  : 白银
//! @LastEditTime : 2023-02-03 17:11:24
//! @FilePath     : /rwaf/src/module/detect/check_web_shell.rs
//! @Description  :
//! @Attention    :
//! @Copyright (c) 2023 by 白银 captain-jparrow@qq.com, All Rights Reserved.

use std::{
    fs::{self, File},
    io::Read,
    path::Path,
};

use execute::Execute;
use lettre::{transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};

pub fn start_check_web_shell() {
    let dst_path = "/home/jack/Desktop/pwn".to_string();
    // let src_ip = "129.226.211.132";
    // let src_username = "root";
    // let src_path = "/home/lighthouse/pwn";

    let program_name = "python3".to_string();
    let use_script = "src/module/detect/check_web_shell.py".to_string();
    // let dst_path = "python3".to_string();

    // let fin_shell = "python3 src/module/detect/check_web_shell.py".to_string() + &" '" + dst_path + &"'";
    // let fin_shell = "scp -r root@129.226.211.132:/home/lighthouse/pwn /home/jack/Desktop/pwm".to_string();

    // println!("{}", check_web_shell(fin_shell));
    // check_web_shell(fin_shell);
    // check_web_shell(program_name, use_script, dst_path);
    // println!("done");
    // break;

    // let path = env::current_dir().unwrap();
    // let path = "src/module/detect";
    // let dir = path.as_ptr().addr().unwrap();
    // for x in dir {
    //     if let Ok(path) = x {
    //        println!("{:?}", path.file_name()); // 该路径下所有文件和文件夹名称
    //        // 是否存在某个文件
    //         if path.file_name().eq("src/module/detect/dan.log") {
    //             rm_rf_dan_log("src/module/detect/dan.log".to_string());
    //         }
    //     }
    // }

    if Path::new("src/module/detect/dan.log").exists() {
        rm_rf_dan_log("src/module/detect/dan.log".to_string());
    }

    // rm_rf_dan_log("src/module/detect/dan.log".to_string());

    check_web_shell(program_name, use_script, dst_path);

    let mut open_dan_log = File::open("src/module/detect/dan.log").unwrap();
    let mut dan_log_content = String::new();
    open_dan_log.read_to_string(&mut dan_log_content).unwrap();
    let dan_log_size = fs::metadata("src/module/detect/dan.log").unwrap().len();

    if dan_log_size > 0 {
        let sender_address = "rwaf <jj1017708679@126.com>".to_string(); //like: NoBody <nobody@domain.tld>
        let receiver_address = "Jack Jparrow <captain-jparrow@qq.com>".to_string(); //like: NoBody <nobody@domain.tld>
        let mail_body =
            dan_log_content + &"\n\n" + &"sent by rwaf(https://github.com/Jack-Jparrow/rwaf.git)";
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
    // println!("{}", dan_log_size);
    // let len_n = len.to_string();

    // println!("{}", len_n);

    // match len {
    //     Ok(0) => {
    //         println!("none")
    //     }
    //     _ => {
    //         print!("{}", check_web_shell(program_name, use_script, dst_path))
    //     }
    // }
}

// fn check_web_shell(fin_shell: String) {
//     execute::shell(fin_shell);
// }

fn check_web_shell(program_name: String, use_script: String, dst_path: String) {
    let mut command = execute::command_args!(program_name, use_script, dst_path);
    let output = command.execute_output().unwrap();

    // match command.status().unwrap().code(){
    //     Some(code) => {
    //         println!("Exit Status: {}", code);
    //     }
    //     None =>{
    //         println!("Process terminated");
    //     }
    // }
    // println!("{:?}", );
    // let res = String::from_utf8(output.stdout).unwrap();
    // print!("{}", res);

    // res.trim().to_string()
}

fn rm_rf_dan_log(log_path: String) {
    let mut command = execute::command_args!("rm", "-rf", log_path);
    let output = command.execute_output().unwrap();

    // match command.status().unwrap().code(){
    //     Some(code) => {
    //         println!("Exit Status: {}", code);
    //     }
    //     None =>{
    //         println!("Process terminated");
    //     }
    // }
    // println!("{:?}", );
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
