<!--
 * @Author       : 白银
 * @Date         : 2023-01-31 21:22:22
 * @LastEditors  : 白银
 * @LastEditTime : 2023-02-17 20:49:45
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
- [ ] 守护进程
- [x] 写入数据库
- [x] 优化反击模块
- [x] 在响应记录数据库中，写入ban掉的ip及当时日期时间，手动恢复日期时间，手动反击日期时间目标ip

# how to use 
`cargo run [OPTIONS] [args]...`

Options:

    -- -h           Show basic help message and exit
    -- -m           Run rwaf/src/main.rs
    -- -re          Run rwaf/src/modules/restore/make_restore.rs, Manually execute the restore procedure
    -- -ct          Run rwaf/src/modules/counterattack/syn_flood.rs, Manually execute the counterattack procedure

ARGS:

    <args>...       If the [OPTIONS] is '-- -ct': 
                        the 1st [args] will be the target's IPv4 address and port, like '127.0.0.1:1234',
                        the 2nd [args] will be the number of threads, like: '100', the program will run in 100 threads

## for example
`cargo run -- -h`

![image](readme.png)