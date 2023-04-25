'''
@Author       : 白银
@Date         : 2023-02-12 16:25:14
@LastEditors  : 白银
@LastEditTime : 2023-02-12 19:42:07
@FilePath     : /rwaf/src/module/warning/port_scan.py
@Description  : 端口扫描
@Attention    : 
@Copyright (c) 2023 by 白银 captain-jparrow@qq.com, All Rights Reserved. 
'''

#!/usr/bin/python
# encoding:utf-8

import socket
# import sys
import threading
import queue

file = open("src/module/warning/.log/port.log", "a", encoding='Utf-8')

def scan():
    while not q.empty():
        port = q.get()
        c = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        if c.connect_ex((host, port)) == 0:
            # print("%s:%d is open" % (host, port))
            wr = "%s:%d is open" % (host, port)
            file.write(wr)
            file.write("\n")
        # file.close()
        c.close()


if __name__ == "__main__":
    # host = sys.argv[1] #作为参数写入
    # thread_num = sys.argv[2]
    #host = '47.94.106.197'
    host = '127.0.0.1'
    thread_num = 100
    q = queue.Queue()
    for port in range(1, 65535):
        q.put(port)

    for i in range(int(thread_num)):
        t = threading.Thread(target=scan)
        t.start()
        t.join()  # 子线程全部运行完了结束进程，以免线程卡死
    
    file.close()
