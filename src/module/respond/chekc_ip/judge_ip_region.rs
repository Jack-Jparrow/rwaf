//! @Author       : 白银
//! @Date         : 2023-05-15 09:50:56
//! @LastEditors  : 白银
//! @LastEditTime : 2023-05-15 17:57:56
//! @FilePath     : /rwaf/src/module/respond/chekc_ip/judge_ip_region.rs
//! @Description  : 根据得到的数据，识别IP Identify IP based on the obtained data
//! @Attention    :
//! @Copyright (c) 2023 by 白银 captain-jparrow@qq.com, All Rights Reserved.

use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::net::Ipv4Addr;

// 根据IP地址查找IP所属的国家或地区，返回包含IP地址、国家或地区和是否属于中国境内的元组
// Find the country or region to which an IP address belongs based on the IP address and return a tuple that contains the IP address, the country or region, and whether it belongs to China.
fn find_region(ip: Ipv4Addr) -> (String, String, bool) {
    let mut if_cn = true; // 标识IP地址是否属于中国境内 Indicates whether the IP address belongs to China
    let mut ip_region = String::new(); // 存储IP地址所属的国家或地区 Stores the country or region to which the IP address belongs

    // 判断是否属于中国境内IP地址段 Judge whether the IP address falls within the scope of China's IP address range
    if !ip.is_private() && !ip.is_loopback() {
        // 打开名字中包含 "delegated-apnic-extended" 的文件 Open the file with "delegated-apnic-extended" in the name
        let re = Regex::new(r"delegated-apnic-extended").unwrap(); // 创建一个正则表达式对象，用于匹配文件名 Create a regular expression object for matching file names
        let path = std::path::PathBuf::from("src/module/respond/cn_ip"); // 定义文件夹路径 Define the folder path
        let file_entries = std::fs::read_dir(&path) // 打开文件夹 Define the folder path
            .unwrap() // 如果失败直接panic Panic if opening fails directly
            .filter_map(Result::ok) // 过滤出所有结果OK的项 Filter out all items whose result is OK
            .map(|entry| entry.path()) // 遍历每个项并返回其路径 Traverse each item and return its path
            .filter(|path| path.is_file() && re.is_match(path.to_string_lossy().as_ref())) // 过滤出所有符合条件的文件 Filter out all files that meet the conditions
            .collect::<Vec<_>>(); // 将所有结果放入一个集合中 Store all results into a vector

        if file_entries.len() != 1 {
            // 如果条件成立，说明没有找到目标文件 If this condition is met, it means that the target file was not found
            return (ip.to_string(), ip_region, if_cn); // 返回元组 Return the tuple
        }

        let file = File::open(file_entries[0].to_owned()).expect("Unable to open file"); // 打开目标文件 Open the target file
        let reader = BufReader::new(file); // 创建文件缓存读取器 Create a file cache reader

        for line in reader.lines() {
            // 遍历文件中每行内容 Traverse each line in the file
            let line = line.expect("Unable to read line"); // 如果失败直接 panic Panics if failed to read the line

            if line.starts_with("apnic|") && line.contains("|ipv4|") {
                // 如果当前行是IP地址信息所在行 If the current line is the line where IP address information is located
                let fields: Vec<&str> = line.split('|').collect(); // 将当前行内容按照‘|’分隔后存储到集合fields中 Store the content of the current line after splitting by '|' into the vector 'fields'
                let start_ip = fields[3].parse::<Ipv4Addr>().expect("Invalid IP address"); // 获取IP地址区间的起始IP地址 Obtain the starting IP address of the IP address range
                let prefix_len = 32
                    - (fields[4].parse::<u32>().expect("Invalid prefix length")).trailing_zeros(); // 获取子网掩码前缀长度 Get the prefix length of the subnet mask

                // 计算结束IP地址 Calculate the ending IP address
                let ip_num: u32 = start_ip.into();
                let end_num: u32 = (!0_u32) >> prefix_len;
                let end_ip: Ipv4Addr = (ip_num | end_num).into();

                if ip >= start_ip && ip <= end_ip {
                    // 如果当前IP地址在该区间内 If the current IP address is within this range
                    ip_region = fields[1].to_string(); // 获取IP所属国家或地区 Obtain the country or region to which the IP belongs
                    match &ip_region[..] {
                        // 如果所属地域是中国大陆或港澳台，则认为其属于中国境内 If the region is Mainland China or Hong Kong, Macao and Taiwan, it is deemed to belong to China
                        "CN" => if_cn = true,
                        "TW" => if_cn = true,
                        "HK" => if_cn = true,
                        "MO" => if_cn = true,
                        _ => if_cn = false,
                    }
                    break; // 找到目标IP地址对应的IP段后，结束遍历 Once the IP address range corresponding to the target IP address is found, stop traversing
                }
            }
        }
    }

    // 判断是否是私有地址或回环地址 Judge whether it is a private or loopback address
    if ip.is_private() {
        ip_region = String::from("private");
        if_cn = true; // 私有地址被认为属于中国大陆IP地址段 Private addresses are deemed to belong to the Mainland China IP address range
    } else if ip.is_loopback() {
        ip_region = String::from("localhost");
        if_cn = true; // 回环地址被认为属于中国大陆IP地址段 Loopback addresses are deemed to belong to the Mainland China IP address range
    }

    (ip.to_string(), ip_region, if_cn)
}

pub fn judge_ip_region_main(ip_str: &str) -> (String, String, bool) {
    // let ip_str = "192.168.1.100"; // 输入想要查询的IP地址
    let ip: Ipv4Addr = ip_str.parse().expect("Invalid IP address");

    let (ip_addr, ip_region, if_cn) = find_region(ip); // 调用find_region函数查询信息 Call the find_region function to query information

    // println!("The IP address {} is from {}, {}", ip_addr, ip_region, if_cn);

    (ip_addr, ip_region, if_cn) // 返回元组 Return the tuple
}
