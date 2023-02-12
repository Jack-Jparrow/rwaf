'''
@Author       : 白银
@Date         : 2023-02-10 14:32:08
@LastEditors  : 白银
@LastEditTime : 2023-02-12 16:02:01
@FilePath     : /rwaf/src/module/warning/init.py
@Description  : 
@Attention    : 
@Copyright (c) 2023 by 白银 captain-jparrow@qq.com, All Rights Reserved. 
'''
# -*- coding:utf-8 -*-
import requests, json, time
def scan_sql(url):
    r = requests.get(url='http://127.0.0.1:8775/task/new')
    task_id = r.json()['taskid']
    sqlmap_set = 'http://127.0.0.1:8775/option/%s/set' % task_id
    sqlmap_url = 'http://127.0.0.1:8775/scan/%s/start' % task_id
    sqlmap_status = 'http://127.0.0.1:8775/scan/%s/status' % task_id
    sqlmap_result = 'http://127.0.0.1:8775/scan/%s/data' % task_id
    set = requests.post(url=sqlmap_set, data=json.dumps({'url': url}), headers={'Content-Type': 'application/json'})
    scans = requests.post(url=sqlmap_url, data=json.dumps({'url': url}), headers={'Content-Type': 'application/json'})
    r = requests.get(sqlmap_status).json()['status']
    print('当前运行状态:{}'.format(r))
    while 1:
        if requests.get(sqlmap_status).json()['status'] == 'running':
            # 当前表示正在跑数据
            time.sleep(10)
            # 每十秒钟请求一次扫描状态
        else:
            print(requests.get(sqlmap_status).json()['status'])
            if 'terminated'==requests.get(sqlmap_status).json()['status']:
                re = requests.get(url=sqlmap_result)
                print('当前网址扫描完毕')
                print(re.json())
                return re.json()

# scan_sql('http://47.94.106.197:1024/admin/admin.php')
scan_sql('http://47.94.106.197:1024/sqlilabs/Less-1/?id=1')