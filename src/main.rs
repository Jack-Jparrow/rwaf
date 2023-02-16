//! @Author       : 白银
//! @Date         : 2023-01-11 20:42:38
//! @LastEditors  : 白银
//! @LastEditTime : 2023-02-16 19:17:30
//! @FilePath     : /rwaf/src/main.rs
//! @Description  :
//! @Attention    :
//! @Copyright (c) 2023 by 白银 captain-jparrow@qq.com, All Rights Reserved.

use mysql::prelude::*;
use mysql::*;
use std::{env, io, thread};

mod module;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query0 = &args.clone()[0];
    match &query0 as &str {
        "--" => {
            let query1 = &args.clone()[1];
            match &query1 as &str {
                "-m" => {
                    let step_1 =
                        thread::spawn(|| module::warning::port_sql_xss::port_sql_xss_main());
                    step_1.join();
                    let step_2 =
                        thread::spawn(|| module::protect::show_watch_res::show_watch_res_main());
                    // step_2.join();
                    let step_3 = thread::spawn(|| {
                        module::detect::check_web_shell::start_check_web_shell_main();
                    });
                    let step_4 = thread::spawn(|| module::respond::stop_ddos::stop_ddos_main());
                    let step_5 = thread::spawn(|| module::protect::make_bak::use_start_make_bak());
                    step_5.join();
                }
                "-h" => output_help(),
                "-ct" => module::counterattack::syn_flood::start_syn(),
                "-re" => module::restore::make_restore::start_make_restore(),
                _ => output_help(),
                // _ => println!("123")
            }
        }
        _ => output_help(),
    }
}

fn output_help() {
    println!("cargo run [OPTIONS] [args]...");
    println!("Options:");
    println!("    -- -h           Show basic help message and exit");
    println!("    -- -m           Run rwaf/src/main.rs");
    println!("    -- -re          Run rwaf/src/modules/restore/make_restore.rs, Manually execute the restore procedure");
    println!("    -- -ct          Run rwaf/src/modules/counterattack/syn_flood.rs, Manually execute the counterattack procedure");
    println!("ARGS:");
    println!("    <args>...       If the [OPTIONS] is '-- -ct', the 1st [args] will be the target's IPv4 address and port, like '127.0.0.1:1234'");
}

fn sql_check() {
    let mut id = String::new();
    io::stdin().read_line(&mut id).expect("err");
    // let id = String::from("root");
    let resu = sql(&mut id);
    // println!("{:?}", resu);
    for r in resu {
        println!("Host = {}, User = {}, Password = {}", r.0, r.1, r.2);
    }
    println!("1234");
}

fn sql(
    str: &mut String,
) -> Vec<(
    std::string::String,
    std::string::String,
    std::string::String,
)> {
    let url = "mysql://username:passwd@ipv4:port/sqlname"; //每次git都要码掉！！！
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();

    let res: Vec<(String, String, String)> = conn
        .query(
            "select Host, User, Password from user where User = \"".to_owned()
                + &str.trim()
                + "\";",
        )
        .unwrap();

    // let qu = "select Host, User, Password from user where User = \"".to_owned() + &str + "\";";
    // println!("{}", qu);

    res

    // println!("{:?}", res);
}
