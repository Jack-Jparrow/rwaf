//! @Author       : 白银
//! @Date         : 2023-01-17 17:53:31
//! @LastEditors  : 白银
//! @LastEditTime : 2023-01-18 15:15:50
//! @FilePath     : /rwaf/src/module/warning/a.rs
//! @Description  : 
//! @Attention    : 
//! @Copyright (c) 2023 by 白银 captain-jparrow@qq.com, All Rights Reserved. 

pub fn dofunc(){
    println!("{}","Welcome to use nets");

    let dev = libpcap::lookup();    

    println!("{}",dev);

    let mut Packet = libpcap::open(dev.as_str());

    libpcap::setfilter(&mut Packet,"tcp port 80");
    while let data = libpcap::next_ex(&mut Packet){
        println!("Packet Length {:?}",Packet.head.len);
        println!("{:?}",Packet);
    }

    libpcap::close(&mut Packet); 
}