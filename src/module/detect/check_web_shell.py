'''
@Author       : 白银
@Date         : 2023-02-02 16:02:12
@LastEditors  : 白银
@LastEditTime : 2023-02-02 19:32:52
@FilePath     : /rwaf/src/module/detect/check_web_shell.py
@Description  : 
@Attention    : 
@Copyright (c) 2023 by 白银 captain-jparrow@qq.com, All Rights Reserved. 
'''

#!/usr/bin/env python
# -*- coding: utf-8 -*-

import os
import re
import smtplib
import sys

# # 设定邮件
# fromaddr = "smtp.qq.com"
# toaddrs = ["voilet@qq.com"]
# username = "voilet"
# password = "xxxxxx"


# 设置白名单
# pass_file = ["api_ucenter.php"]

# # 定义发送邮件函数


# def sendmail(toaddrs, sub, content):
#     '发送邮件模块'
#     # Add the From: and To: headers at the start!
#     msg = ("From: %s\r\nTo: %s\r\nSubject: %s\r\n\r\n"
#            % (fromaddr, ", ".join(toaddrs), sub))
#     msg += content
#     server = smtplib.SMTP('mail.funshion.com', 25,)
#     server.login(username, password)
#     server.sendmail(fromaddr, toaddrs, msg)
#     server.quit()


# 设置搜索特征码
rulelist = [
    '(\$_(GET|POST|REQUEST)\[.{0,15}\]\(\$_(GET|POST|REQUEST)\[.{0,15}\]\))',
    '(base64_decode\([\'"][\w\+/=]{200,}[\'"]\))',
    'eval\(base64_decode\(',
    '(eval\(\$_(POST|GET|REQUEST)\[.{0,15}\]\))',
    '(assert\(\$_(POST|GET|REQUEST)\[.{0,15}\]\))',
    '(\$[\w_]{0,15}\(\$_(POST|GET|REQUEST)\[.{0,15}\]\))',
    '(wscript\.shell)',
    '(gethostbyname\()',
    '(cmd\.exe)',
    '(shell\.application)',
    '(documents\s+and\s+settings)',
    '(system32)',
    '(serv-u)',
    '(提权)',
    '(phpspy)',
    '(后门)',
    '(webshell)',
    '(Program\s+Files)',
    'www.phpdp.com',
    'phpdp',
    'PHP神盾',
    'decryption',
    'Ca3tie1',
    'GIF89a',
    'IKFBILUvM0VCJD\/APDolOjtW0tgeKAwA',
    '\'e\'\.\'v\'\.\'a\'\.\'l\'',
]


def Scan(path):
    for root, dirs, files in os.walk(path):
        for filespath in files:
            isover = False
            if '.' in filespath:
                ext = filespath[(filespath.rindex('.')+1):]
                if ext == "html" or ext == "htm" or ext == "php" or ext == "php3" or ext == "php4" or ext == "php5" or ext == "asp" or ext == "aspx" or ext == "jsp" or ext == "jspx" or ext == "cfm":#and filespath not in pass_file:
                    file = open(os.path.join(root, filespath), encoding = 'Utf-8')
                    file2 = open("src/module/detect/dan.log", "a", encoding = 'Utf-8')
                    filestr = file.read()
                    file.close()
                    for rule in rulelist:
                        result = re.compile(rule).findall(filestr)
                        if result:
                            # file = open("src/module/detect/dan.log", "a", encoding = 'Utf-8')
                            wr = str("文件：" + os.path.join(root, filespath) + "\n" + "恶意代码：" + str(result[0]) + "\n")
                            file2.write(wr)
                            file2.close()
                            # print('文件：'+os.path.join(root, filespath))
                            # print('恶意代码：'+str(result[0]))
                            # print('\n\n'sendmail(toaddrs, "增值发现恶意代码", '文件：'+os.path.join(root,filespath)+"\n" + '恶意代码：'+str(result[0])))
                            break

try:
    if os.path.lexists(str(sys.argv[1])):
        # print('开始扫描：' + str(sys.argv[1]))
        # print('        可疑文件         ')
        # print('########################################')
        Scan(str(sys.argv[1]))
        # print('提示：扫描完成--~')
    else:
        print('提示：指定的扫描目录不存在--- ')
except IndexError:
    print("请指定扫描文件目录")
