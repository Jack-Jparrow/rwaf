//! @Author       : 白银
//! @Date         : 2023-01-11 20:42:38
//! @LastEditors  : 白银
//! @LastEditTime : 2023-01-13 19:47:01
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
    module::warning::a::hello();
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
