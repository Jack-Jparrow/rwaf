<!--
 * @Author       : 白银
 * @Date         : 2023-01-31 21:22:22
 * @LastEditors  : 白银
 * @LastEditTime : 2023-02-16 19:15:29
 * @FilePath     : /rwaf/README.md
 * @Description  : 
 * @Attention    : 
 * @Copyright (c) 2023 by 白银 captain-jparrow@qq.com, All Rights Reserved.
-->

# waf based on rust

`openssl` & `bison` & `flex` & `libpcap` & `python-requests` & `sqlmap` needed

`iptables` or `firewalld` needed

```rustup toolchain install nightly```

make sure that you've been able to connect to the bak_server by `ssh-key` before using the `restore` module

**This is a graduation project from [School of Computer Science of THNU](https://jsjxy.thnu.edu.cn)**

***Not yet completed, under development***

# todo
- [] 守护进程
- [] 写入数据库
- [] 优化反击模块
- [] 忘了。。。
- [] 在响应记录数据库中，写入ban掉的ip及当时日期时间，手动恢复日期时间，手动反击日期时间目标ip