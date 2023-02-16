//! @Author       : 白银
//! @Date         : 2023-02-16 14:52:23
//! @LastEditors  : 白银
//! @LastEditTime : 2023-02-16 15:25:58
//! @FilePath     : /rwaf/src/module/use_sm3.rs
//! @Description  : sm3 encrypt
//! @Attention    : 
//! @Copyright (c) 2023 by 白银 captain-jparrow@qq.com, All Rights Reserved. 

use rcrypto::Digest;
use rcrypto::SM3;

pub fn sm3_main(text: String) -> String{
    
    // let text = "2020-11-12 21:22:33 deal with stop ddos".to_string();
    let mut hasher = SM3::new();
    let mut digest = Vec::new();
    hasher.write(text.as_bytes());
    hasher.checksum(&mut digest);
    // println!("{} => {}",text, cvt_bytes_to_str(digest.as_slice()));
    cvt_bytes_to_str(digest.as_slice())
}

fn cvt_bytes_to_str(b: &[u8]) -> String {
    let mut s= String::new();
    for &ele in b.iter() {
        let e = format!("{:02x}", ele);
        s.push_str(e.as_str());
    }
    s
}