//! @Author       : 白银
//! @Date         : 2023-01-29 19:14:02
//! @LastEditors  : 白银
//! @LastEditTime : 2023-01-29 21:14:07
//! @FilePath     : /rwaf/src/module/protect/watch_memory.rs
//! @Description  : 
//! @Attention    : 
//! @Copyright (c) 2023 by Jack-Jparrow captain-jparrow@qq.com, All Rights Reserved. 

use std::process::{Command, Stdio};

pub fn output_get_mem_state(){
    let mem_state = get_mem_state();
    let total_mem: f64 = total_mem(&mem_state).trim().parse().unwrap();
    let avaa_mem: i32 = ava_mem(&mem_state).trim().parse().unwrap();
    let ava_memm: f64 = ava_mem(&mem_state).clone().trim().parse().unwrap();

    match avaa_mem {
        0 => println!("memory full"),
        _ => {
            // let ava_memm: f64 = ava_mem(&mem_state).trim().parse().unwrap();
            let ava: f64 =  ava_memm / total_mem;
            println!("memory occupy: {:.1}%", (1.0 - ava) * 100.0)
        }
    }
    
    // println!("{}, {}", total_mem(&mem_state), ava_mem(&mem_state));
}

fn get_mem_state() -> String {
    let output1 = Command::new("free").stdout(Stdio::piped()).spawn().unwrap();
    let output2 = Command::new("grep").arg("Mem").stdin(output1.stdout.unwrap()).stdout(Stdio::piped()).spawn().unwrap();
    let output3 = Command::new("awk").arg("{print $2, $7}").stdin(output2.stdout.unwrap()).output().unwrap();
    let res = String::from_utf8(output3.stdout).unwrap();

    res.trim().to_string()
}

fn total_mem(s: &String) -> &str {
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

fn ava_mem(s: &String) -> &str {
    let len = s.trim().chars().count();
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i + 1..len];
        }
    }

    // s.len()
    &s[..]
}
