//! @Author       : 白银
//! @Date         : 2023-01-30 21:47:28
//! @LastEditors  : 白银
//! @LastEditTime : 2023-01-31 21:06:58
//! @FilePath     : /rwaf/src/module/protect/show_watch_res.rs
//! @Description  :
//! @Attention    :
//! @Copyright (c) 2023 by ${git_name_email}, All Rights Reserved.

use std::thread;

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
        let t = thread::spawn(move ||{
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
        });

        thread::sleep_ms(6000);
    }
    // println!("{}", super::watch::watch_disk::output_get_disk_state());
    // super::watch::watch_cpu::split_from_res_date_time_cpu_state();
}
