//! @Author       : 白银
//! @Date         : 2023-01-11 20:42:38
//! @LastEditors  : 白银
//! @LastEditTime : 2023-02-17 20:38:02
//! @FilePath     : /rwaf/src/main.rs
//! @Description  :
//! @Attention    :
//! @Copyright (c) 2023 by 白银 captain-jparrow@qq.com, All Rights Reserved.

use std::{env, thread};

mod module;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query0 = &args.clone()[0];
    // println!("********");
    // println!("{:?},{}", query0, "a");
    // println!("********");
    match &query0 as &str {
        "target/debug/rwaf " => output_help(),
        "target/debug/rwaf" => {
            let query1 = &args.clone()[1];
            // println!("********");
            // println!("{:?}", query1);
            // println!("********");
            match &query1 as &str {
                "-m" => {
                    let step_1 =
                        thread::spawn(|| module::warning::port_sql_xss::port_sql_xss_main());
                    step_1.join().unwrap();
                    let _step_2 =
                        thread::spawn(|| module::protect::show_watch_res::show_watch_res_main());
                    // _step_2.join();
                    let _step_3 = thread::spawn(|| {
                        module::detect::check_web_shell::start_check_web_shell_main();
                    });
                    let _step_4 = thread::spawn(|| module::respond::stop_ddos::stop_ddos_main());
                    let step_5 = thread::spawn(|| module::protect::make_bak::use_start_make_bak());
                    step_5.join().unwrap();
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
    println!("    <args>...       If the [OPTIONS] is '-- -ct': ");
    println!("                        the 1st [args] will be the target's IPv4 address and port, like '127.0.0.1:1234',");
    println!("                        the 2nd [args] will be the number of threads, like: '100', the program will run in 100 threads");
}
