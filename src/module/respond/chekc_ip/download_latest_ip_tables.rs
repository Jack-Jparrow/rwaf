//! @Author       : 白银
//! @Date         : 2023-05-11 19:21:49
//! @LastEditors  : 白银
//! @LastEditTime : 2023-05-12 15:22:15
//! @FilePath     : /rwaf/src/module/respond/chekc_ip/download_latest_ip_tables.rs
//! @Description  : 从APNIC获得IP地址分配表 Get IP address allocation table from APNIC
//! @Attention    :
//! @Copyright (c) 2023 by 白银 captain-jparrow@qq.com, All Rights Reserved.

use flate2::read::MultiGzDecoder;
use std::error::Error;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};

// 从apnic获取IP地址分配表，下载并解压
// Download and extract IP address allocation table from apnic
fn download_and_extract_file(date: &str) -> Result<(), Box<dyn Error>> {
    // 根据日期拼接出下载地址、文件名和压缩文件名（apnic每天会给出前一天的IP地址分配情况） Construct the download URL, file name and compressed file name based on the date (apnic publishes statistics for the previous day's IP address assignments)
    let url = format!(
        "https://ftp.apnic.net/stats/apnic/{}/delegated-apnic-extended-{}.gz",
        &date[..4],
        &date
    );
    let file_name = format!("delegated-apnic-extended-{}", date);
    let gz_file_name = format!("{}.gz", file_name);

    // 定义存放文件的目录路径，如果目录不存在，则创建该目录 Define the directory path to store the files. If the directory does not exist, create it.
    let dir_path = Path::new("src/module/respond/cn_ip");
    if !dir_path.exists() {
        fs::create_dir_all(dir_path)?;
    }

    // 定义文件路径和压缩文件路径 Define the file path and compressed file path
    let file_path = dir_path.join(&file_name);
    let gz_file_path = dir_path.join(&gz_file_name);

    // 使用wget命令下载文件 Use wget command to download the file
    let output = Command::new("wget")
        .arg("-P")
        .arg(&dir_path)
        .arg(&url)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;
    if !output.status.success() {
        // 如果下载进程非正常退出，则输出错误信息并返回错误 If the download process exits abnormally, output the error message and return an error
        return Err(format!(
            "Download file failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    // 解压文件并写入到新文件中 Extract the downloaded gz file
    if file_path.exists() {
        // 如果原文件存在，则删除它 If the original file exists, delete it
        fs::remove_file(&file_path)?;
    }
    let mut gz_file = File::open(&gz_file_path)?; // 打开压缩文件 Open the compressed file
    let mut decoder = MultiGzDecoder::new(&mut gz_file); // 创建一个MultiGzDecoder实例，用于解压gzip格式的数据 Create a MultiGzDecoder instance to decompress gzip formatted data
    let mut file = File::create(&file_path)?; // 创建新文件  Create the new file
    let mut buf = [0; 1024];
    loop {
        match decoder.read(&mut buf) {
            // 每次读取1024字节的数据 Read 1024 bytes of data each time
            Ok(0) => break, // 如果没有数据被读取，则退出循环 Exit the loop if no data is read
            Ok(n) => file.write_all(&buf[..n])?, // 如果有数据被读取，则将其写入新文件中 Write the data to the new file if any data is read
            Err(e) => return Err(e.into()), // 如果有错误发生，则返回错误信息 Return an error message if any error occurs
        }
    }

    Ok(()) // 返回成功消息 Return a success message
}

// 删除旧文件
// Delete old files
fn delete_previous_files(date: &str) -> Result<(), Box<dyn Error>> {
    // let prev_day = chrono::DateTime::parse_from_str(date, "%Y%m%d")? - chrono::Duration::days(1);
    // 定义文件目录路径，读取该路径下的所有文件 Define the file directory path and read all the files in the directory
    let dir_path = Path::new("src/module/respond/cn_ip");
    let files = fs::read_dir(dir_path)?;

    // 遍历每个文件，如果发现文件名包含传入的日期，则删除该文件 Traverse each file, if the file name contains the passed in date, delete that file
    for file in files {
        let file = file?.path();
        if let Some(file_name) = file.file_name().and_then(|n| n.to_str()) {
            if file.is_file() && file_name.contains(&date) {
                fs::remove_file(file)?; // 删除文件 Delete the file
            }
        }
    }

    Ok(()) // 返回成功消息 Return a success message
}

// 更新IP地址分配表
// Update IP address allocation tables
fn update_cn_ip_files() -> Result<(), Box<dyn Error>> {
    // 获取前一天和前两天的日期 Get the date of the previous day and the day before that
    let prev_day = chrono::Utc::now() - chrono::Duration::days(1);
    // println!("prev_day-----{}", &prev_day.format("%Y%m%d"));
    let prev_prev_day = prev_day - chrono::Duration::days(1);
    // println!("prev_prev_day-----{}", &prev_prev_day.format("%Y%m%d"));

    // 下载并提取最新的IP地址分配表（日期为前一天） Download and extract the latest IP address allocation table (from the day before)
    match download_and_extract_file(&prev_day.format("%Y%m%d").to_string()) {
        // 根据前一天的日期下载最新的IP地址分配表 Download the latest IP address allocation table based on the previous day's date
        Ok(_) => (),
        Err(e) => {
            println!("Download file failed: {}", e);
            return Err(e.to_string().into()); // 如果下载失败，则返回错误信息 If the download fails, return an error message
        }
    };

    // 删除前一天的获得的旧文件（日期为前两天） Delete old files obtained from the previous day (date from the previous two days)
    match delete_previous_files(&prev_prev_day.format("%Y%m%d").to_string()) {
        // 根据前两天的日期删除旧文件 Delete old files based on the dates from the previous two days
        Ok(_) => (),
        Err(e) => {
            println!("Delete file failed: {}", e);
            return Err(e.to_string().into()); // 如果删除失败，则返回错误信息 If the deletion fails, return an error message
        }
    };

    Ok(())
}

pub fn download_latest_ip_tables_main() -> Result<(), Box<dyn Error>> {
    update_cn_ip_files()?;
    Ok(())
}
