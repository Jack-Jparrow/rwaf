'''
@Author       : 白银
@Date         : 2023-02-08 15:25:28
@LastEditors  : 白银
@LastEditTime : 2023-02-12 20:27:16
@FilePath     : /rwaf/src/module/warning/sql.py
@Description  : sql注入检测，简单的报错注入
@Attention    :
@Copyright (c) 2023 by 白银 captain-jparrow@qq.com, All Rights Reserved.
'''

# -*- coding:utf-8 -*-
import requests
import sys
payloads = ("'", "')", "';", '"', '")', '";', "--", "-0", "||1=1", ") AND 1998=1532 AND (5526=5526", " AND 5434=5692%23", " %' AND 5268=2356 AND '%'='",
            " ') AND 6103=4103 AND ('vPKl'='vPKl", " ' AND 7738=8291 AND 'UFqV'='UFqV", '`', '`)', '`;', '\\', "%27", "%%2727", "%25%27", "%60", "%5C")
sql_errors = {'SQL syntax': 'mysql', 'syntax to use near': 'mysql', 'MySQLSyntaxErrorException': 'mysql', 'valid MySQL result': 'mysql', 'Access Database Engine': 'Access', 'JET Database Engine': 'Access',
              'Microsoft Access Driver': 'Access', 'SQLServerException': 'mssql', 'SqlException': 'mssql', 'SQLServer JDBC Driver': 'mssql', 'Incorrect syntax': 'mssql', 'MySQL Query fail': 'mysql'}

file = open("src/module/warning/.log/sql.log", "w", encoding='Utf-8')


def CheckSql(url):
    for payload in payloads:
        urlli = url+payload
        try:
            r = requests.get(urlli).content
            for k, v in sql_errors.items():
                if k.encode() in r:
                    # wr = str('存在{}数据库注入'.format(v))
                    # file.write(wr)
                    # file.write("\n")
                    # break
                    # file.close()
                    # break
                    return '存在{}数据库注入'.format(v)
        except Exception as e:
            print(e)
            # print("不存在数据库注入")
            # wr = str('不存在数据库注入')
            # file.write(wr)
            # file.close()
            # return '不存在数据库注入'
            # break


# url = 'http://47.94.106.197:1024/sqlilabs/Less-3/?id=1'
# url = 'http://47.94.106.197:1024/admin/admin.php/?id=1'
# url = sys.argv[1]
# # print(CheckSql(url))
# CheckSql(url)
# file.close()

if __name__ == '__main__':
    # file = open("src/module/warning/xss.log", "a", encoding='Utf-8')
    url = sys.argv[1]
# print(CheckSql(url))
    wr = str(CheckSql(url))
    file.write(wr)
    # CheckSql(url)
    file.close()
