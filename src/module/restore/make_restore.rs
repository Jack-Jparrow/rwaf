//! @Author       : 白银
//! @Date         : 2023-02-01 20:23:32
//! @LastEditors  : 白银
//! @LastEditTime : 2023-02-01 21:12:40
//! @FilePath     : /rwaf/src/module/restore/make_restore.rs
//! @Description  : 从备份服务器恢复
//! @Attention    :
//! @Copyright (c) 2023 by 白银 captain-jparrow@qq.com, All Rights Reserved.

pub fn start_make_restore() {
    let dst_path = "/home/jack/Desktop/pwm";
    let src_ip = "129.226.211.132";
    let src_username = "root";
    let src_path = "/home/lighthouse/pwn";

    let fin_shell =
        "scp -r ".to_string() + src_username + &"@" + src_ip + &":" + src_path + &" " + dst_path;
    // let fin_shell = "scp -r root@129.226.211.132:/home/lighthouse/pwn /home/jack/Desktop/pwm".to_string();

    // println!("{}", make_restore(fin_shell));
    make_restore(fin_shell);
    // println!("done");
    // break;
}

// fn make_restore(fin_shell: String) {
//     execute::shell(fin_shell);
// }

fn make_restore(fin_shell: String) {
    let mut command = execute::shell(fin_shell);
    // let output = Command::new("echo").args(["123".to_string()]).output().unwrap();

    match command.status().unwrap().code(){
        Some(code) => {
            println!("Exit Status: {}", code);
        }
        None =>{
            println!("Process terminated");
        }
    }
    // let res = String::from_utf8(output.stdout).unwrap();

    // res.trim().to_string()
}
