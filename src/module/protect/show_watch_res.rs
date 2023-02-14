//! @Author       : 白银
//! @Date         : 2023-01-30 21:47:28
//! @LastEditors  : 白银
//! @LastEditTime : 2023-02-14 18:55:10
//! @FilePath     : /rwaf/src/module/protect/show_watch_res.rs
//! @Description  :
//! @Attention    :
//! @Copyright (c) 2023 by 白银 captain-jparrow@qq.com, All Rights Reserved.

use std::{thread, time::Duration};

// mod watch;
pub fn show_watch_res_main() {
    // println!("");
    // println!(
    //     "{} {} {} {} {} {} {}",
    //     super::watch::watch_date_time_cpu::output_get_date_state(),
    //     super::watch::watch_date_time_cpu::output_get_time_state(),
    //     super::watch::watch_date_time_cpu::output_get_cpu_state(),
    //     super::watch::watch_memory::output_get_mem_state(),
    //     super::watch::watch_disk::output_get_disk_state(),
    //     super::watch::watch_net::output_get_net_state_receive(),
    //     super::watch::watch_net::output_get_net_state_send()
    // );
    loop {
        let t = thread::spawn(move || {
            println!(
                "{} {} {} {} {} {} {}",
                super::watch::watch_date_time_cpu::output_get_date_state(),
                super::watch::watch_date_time_cpu::output_get_time_state(),
                super::watch::watch_date_time_cpu::output_get_cpu_state(),
                super::watch::watch_memory::output_get_mem_state(),
                super::watch::watch_disk::output_get_disk_state(),
                super::watch::watch_net::output_get_net_state_receive(),
                super::watch::watch_net::output_get_net_state_send()
            );

            let date_state = super::watch::watch_date_time_cpu::output_get_date_state();
            let time_state = super::watch::watch_date_time_cpu::output_get_time_state();
            let cpu_state = super::watch::watch_date_time_cpu::output_get_cpu_state();
            let mem_state = super::watch::watch_memory::output_get_mem_state();
            let disk_state = super::watch::watch_disk::output_get_disk_state();
            let net_state_receive = super::watch::watch_net::output_get_net_state_receive();
            let net_state_send = super::watch::watch_net::output_get_net_state_send();

            write_to_state_sql();
        });

        thread::sleep(Duration::from_secs(60)); //do every 60s
    }
    // println!("{}", super::watch::watch_disk::output_get_disk_state());
    // super::watch::watch_cpu::split_from_res_date_time_cpu_state();
}

fn write_to_state_sql() {
    todo!()
}
