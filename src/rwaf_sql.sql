/**
 * @Author       : 白银
 * @Date         : 2023-02-15 17:14:14
 * @LastEditors  : 白银
 * @LastEditTime : 2023-02-15 19:50:47
 * @FilePath     : /rwaf/src/rwaf_sql.sql
 * @Description  : 
 * @Attention    : 
 * @Copyright (c) 2023 by 白银 captain-jparrow@qq.com, All Rights Reserved. 
 */

CREATE DATABASE IF NOT EXISTS rwaf DEFAULT CHARACTER SET utf8;

USE rwaf;

-- show tables;
CREATE TABLE IF NOT EXISTS `backup_log` (
    `date` DATE COMMENT 'date',
    `time` TIME COMMENT 'time',
    `event` VARCHAR (64) COMMENT 'event'
);

CREATE TABLE IF NOT EXISTS `system_monitor` (
    `date` DATE COMMENT 'date',
    `time` TIME COMMENT 'time',
    `cpu_state` VARCHAR (64) COMMENT 'cpu_state',
    `mem` VARCHAR (64) COMMENT 'mem',
    `disk` VARCHAR (64) COMMENT 'disk',
    `net_receive` VARCHAR (64) COMMENT 'net_receive',
    `net_send` VARCHAR (64) COMMENT 'net_send',
    `cpu` VARCHAR (64) COMMENT 'cpu'
);

CREATE TABLE IF NOT EXISTS `webshell_log` (
    `date` DATE COMMENT 'date',
    `time` TIME COMMENT 'time',
    `event` VARCHAR (64) COMMENT 'event',
    `if_webshell` VARCHAR (64) COMMENT 'if_webshell',
    `if_send_email` VARCHAR (64) COMMENT 'if_send_email'
);

CREATE TABLE IF NOT EXISTS `black_ip` (
    `date` DATE COMMENT 'date',
    `time` TIME COMMENT 'time',
    `black_ip` VARCHAR (64) COMMENT 'black_ip'
);

CREATE TABLE IF NOT EXISTS `respond_log` (
    `date` DATE COMMENT 'date',
    `time` TIME COMMENT 'time',
    `event` VARCHAR (64) COMMENT 'event',
    `black_ip` VARCHAR (64) COMMENT 'black_ip',
    `counteratack_ip` VARCHAR (64) COMMENT 'counteratack_ip'
);