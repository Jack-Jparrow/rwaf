//! @Author       : 白银
//! @Date         : 2023-02-01 19:41:02
//! @LastEditors  : 白银
//! @LastEditTime : 2023-02-01 20:19:41
//! @FilePath     : /rwaf/src/module/protect/make_bak.rs
//! @Description  : 备份文件
//! @Attention    :
//! @Copyright (c) 2023 by Jack-Jparrow captain-jparrow@qq.com, All Rights Reserved.

use std::{thread, time::Duration};

pub fn use_start_make_bak() {
    loop {
        let t = thread::spawn(move || start_make_bak());

        thread::sleep(Duration::from_secs(5184000)); //24h备份一次

        // t.join().unwrap();
        // thread::sleep(Duration::from_secs(60));
    }
}

pub fn start_make_bak() {
    let src_path = "/home/jack/Desktop/pwn";
    let dst_ip = "129.226.211.132";
    let dst_username = "root";
    let dst_path = "/home/lighthouse";

    let fin_shell =
        "scp -r ".to_string() + src_path + &" " + dst_username + &"@" + dst_ip + &":" + dst_path;

    // println!("{}", make_bak(fin_shell));
    make_bak(fin_shell);
    // println!("done");
    // break;
}

fn make_bak(fin_shell: String) {
    execute::shell(fin_shell);
}

// fn make_bak(fin_shell: String) {
//     let mut command = execute::shell(fin_shell);
//     // let output = Command::new("echo").args(["123".to_string()]).output().unwrap();

//     match command.status().unwrap().code(){
//         Some(code) => {
//             println!("Exit Status: {}", code);
//         }
//         None =>{
//             println!("Process terminated");
//         }
//     }
//     // let res = String::from_utf8(output.stdout).unwrap();

//     // res.trim().to_string()
// }
