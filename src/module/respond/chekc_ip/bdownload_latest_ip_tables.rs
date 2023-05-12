//! @Author       : 白银
//! @Date         : 2023-05-11 19:21:49
//! @LastEditors  : 白银
//! @LastEditTime : 2023-05-11 21:28:02
//! @FilePath     : /rwaf/src/module/respond/chekc_ip/download_latest_ip_tables.rs
//! @Description  : 
//! @Attention    : 
//! @Copyright (c) 2023 by 白银 captain-jparrow@qq.com, All Rights Reserved. 

use std::error::Error;
use std::fmt::{self, Display};
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use flate2::read::GzDecoder;
use reqwest::header::{self, HeaderMap, HeaderValue};
use chrono::{NaiveDate, Utc};
use reqwest::header::ACCEPT_ENCODING;
use reqwest::Client;

fn get_current_date_time() -> NaiveDate {
    Utc::now().naive_utc().date()
}

#[derive(Debug)]
struct DownloadError(String);

impl Display for DownloadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "下载文件失败：{}", self.0)
    }
}

impl Error for DownloadError {}

impl From<reqwest::Error> for DownloadError {
    fn from(e: reqwest::Error) -> Self {
        DownloadError(format!("下载文件失败：{}", e))
    }
}

impl From<std::io::Error> for DownloadError {
    fn from(e: std::io::Error) -> Self {
        DownloadError(format!("下载文件失败：{}", e))
    }
}

fn download_and_extract_file(date: NaiveDate) -> Result<PathBuf, DownloadError> {
    let url = format!(
        "https://ftp.apnic.net/stats/apnic/{}/delegated-apnic-extended-{}.gz",
        date.format("%Y"),
        date.format("%Y%m%d")
    );
    let file_name = format!("delegated-apnic-extended-{}", date.format("%Y%m%d"));
    let gz_file_name = format!("{}.gz", file_name);
    let mut headers = HeaderMap::new();
    headers.append(ACCEPT_ENCODING, HeaderValue::from_static("gzip, deflate, br"));
    let client = Client::new();
    let response = client.get(&url).send()?;
    if response.status().is_success() {
        let mut content = Vec::new();
        response.copy_to(&mut content)?;

        let mut gz_file_path = Path::new("src/respond/cn_ip").join(&gz_file_name);
        std::fs::write(&gz_file_path, content)?;

        let mut buffer = String::new();
        GzDecoder::new(&content[..]).read_to_string(&mut buffer)?;

        let file_path = Path::new("src/respond/cn_ip").join(&file_name);
        std::fs::write(&file_path, buffer)?;
        Ok(file_path)
    } else {
        Err(DownloadError(format!(
            "下载文件失败，响应状态：{}",
            response.status()
        )))
    }
}

fn update_cn_ip_files() -> Result<(), Box<dyn Error>> {
    let now = get_current_date_time();
    if now.hour() == 0 {
        // 每天的0:00(UTC+8)查询前一天的日期
        let prev_day = now - chrono::Duration::days(1);
        let file_name_prefix = format!("delegated-apnic-extended-{}", prev_day.format("%Y%m%d"));
        let dir_path = Path::new("src/respond/cn_ip/");
        let mut files_to_delete = Vec::new();
        for entry in dir_path.read_dir()? {
            let entry = entry?;
            let file_name = entry.file_name().to_str().unwrap().to_owned();
            if file_name.starts_with("delegated-apnic-extended") && file_name != file_name_prefix {
                files_to_delete.push(entry.path());
                let gz_file_path = Path::new("src/respond/cn_ip/").join(format!("{}.gz", file_name));
                if gz_file_path.exists() {
                    files_to_delete.push(gz_file_path);
                }
            }
        }
        for file_path in files_to_delete {
            std::fs::remove_file(&file_path)?;
        }
        let old_file_path = Path::new("src/respond/cn_ip/").join(format!("delegated-apnic-extended-{}.gz", prev_day.format("%Y%m%d")));
        if old_file_path.exists() {
            match download_and_extract_file(prev_day) {
                Ok(_) => std::fs::remove_file(&old_file_path)?,
                Err(e) => {
                    println!("下载文件失败: {}", e);
                    return Err(Box::new(e));
                }
            };
        } else {
            match download_and_extract_file(prev_day) {
                Ok(_) => (),
                Err(e) => {
                    println!("下载文件失败: {}", e);
                    return Err(Box::new(e));
                }
            };
        }
    } else {
        let file_name_prefix = format!("delegated-apnic-extended-{}", now.format("%Y%m%d"));
        let dir_path = Path::new("src/respond/cn_ip/");
        let mut file_exists = false;
        for entry in dir_path.read_dir()? {
            let entry = entry?;
            let file_name = entry.file_name().to_str().unwrap().to_owned();
            if file_name.starts_with("delegated-apnic-extended") && file_name == file_name_prefix {
                file_exists = true;
                break;
            }
        }
        if !file_exists {
            let old_file_path = Path::new("src/respond/cn_ip/").join(format!("delegated-apnic-extended-{}.gz", now.format("%Y%m%d")));
            if old_file_path.exists() {
                match download_and_extract_file(now) {
                    Ok(_) => std::fs::remove_file(&old_file_path)?,
                    Err(e) => {
                        println!("下载文件失败: {}", e);
                        return Err(Box::new(e));
                    }
                };
            } else {
                match download_and_extract_file(now) {
                    Ok(_) => (),
                    Err(e) => {
                        println!("下载文件失败: {}", e);
                        return Err(Box::new(e));
                    }
                };
            }
        }
    }

    Ok(())
}

fn delete_old_cn_ip_files() -> Result<(), Box<dyn Error>> {
    let dir_path = Path::new("src/respond/cn_ip/");
    let mut files_to_delete = Vec::new();
    for entry in dir_path.read_dir()? {
        let entry = entry?;
        let file_name = entry.file_name().to_str().unwrap().to_owned();
        if file_name.starts_with("delegated-apnic-extended") && file_name != "delegated-apnic-extended-latest.gz" {
            files_to_delete.push(entry.path());
            let gz_file_path = Path::new("src/respond/cn_ip/").join(format!("{}.gz", file_name));
            if gz_file_path.exists() {
                files_to_delete.push(gz_file_path);
            }
        }
    }
    for file_path in files_to_delete {
        std::fs::remove_file(&file_path)?;
    }
    Ok(())
}

pub fn download_latest_ip_tables_main() -> Result<(), Box<dyn Error>> {
    update_cn_ip_files()?;
    delete_old_cn_ip_files()?;
    Ok(())
}