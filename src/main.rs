//! @Author       : 白银
//! @Date         : 2023-01-11 20:42:38
//! @LastEditors  : 白银
//! @LastEditTime : 2023-01-30 22:23:21
//! @FilePath     : /rwaf/src/main.rs
//! @Description  : 
//! @Attention    : 
//! @Copyright (c) 2023 by Jack-Jparrow captain-jparrow@qq.com, All Rights Reserved. 

use mysql::prelude::*;
use mysql::*;
use std::io;

mod module;

fn main() {
    // println!("Hello, world!");
    // loop {
    //     println!("{:?}", module::warning::a::get_available_port());
    // }
    // module::warning::zhuabao::dofunc();
    // module::counterattack::syn_flood::start_syn();
    // module::protect::watch::watch_memory::output_get_mem_state();
    // module::protect::watch::watch_cpu::output_get_cpu_state();
    // module::protect::watch::watch_time::output_get_time();
    // module::protect::watch::watch_date::output_get_day_state();
    module::protect::show_watch_res::show_watch_res_main();
    // println!("{:?}", res);
    // sql_check();
    
}

fn sql_check(){
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
    let url = "mysql://root:Sa123@47.94.106.197:3306/mysql"; //每次git都要码掉！！！
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
