/**
 * @Author       : 白银
 * @Date         : 2023-02-15 17:14:14
 * @LastEditors  : 白银
 * @LastEditTime : 2023-02-16 15:36:38
 * @FilePath     : /rwaf/src/rwaf_sql.sql
 * @Description  : 
 * @Attention    : 
 * @Copyright (c) 2023 by 白银 captain-jparrow@qq.com, All Rights Reserved. 
 */

CREATE DATABASE IF NOT EXISTS rwaf DEFAULT CHARACTER SET utf8;

USE rwaf;

-- show tables;
CREATE TABLE IF NOT EXISTS `backup_log` (
    `event_id` VARCHAR (128) COMMENT 'event_id',
    `date` DATE COMMENT 'date',
    `time` TIME COMMENT 'time',
    `event` VARCHAR (128) COMMENT 'event',
    PRIMARY KEY (`event_id`)
) ENGINE = INNODB DEFAULT CHARSET = utf8;

CREATE TABLE IF NOT EXISTS `system_monitor` (
    `event_id` VARCHAR (128) COMMENT 'event_id',
    `date` DATE COMMENT 'date',
    `time` TIME COMMENT 'time',
    `cpu_state` VARCHAR (128) COMMENT 'cpu_state',
    `mem` VARCHAR (128) COMMENT 'mem',
    `disk` VARCHAR (128) COMMENT 'disk',
    `net_receive` VARCHAR (128) COMMENT 'net_receive',
    `net_send` VARCHAR (128) COMMENT 'net_send',
    `cpu` VARCHAR (128) COMMENT 'cpu',
    PRIMARY KEY (`event_id`)
) ENGINE = INNODB DEFAULT CHARSET = utf8;

CREATE TABLE IF NOT EXISTS `webshell_log` (
    `event_id` VARCHAR (128) COMMENT 'event_id',
    `date` DATE COMMENT 'date',
    `time` TIME COMMENT 'time',
    `event` VARCHAR (128) COMMENT 'event',
    `if_webshell` VARCHAR (128) COMMENT 'if_webshell',
    `if_send_email` VARCHAR (128) COMMENT 'if_send_email',
    PRIMARY KEY (`event_id`)
) ENGINE = INNODB DEFAULT CHARSET = utf8;

CREATE TABLE IF NOT EXISTS `black_ip` (
    `event_id` VARCHAR (128) COMMENT 'event_id',
    `date` DATE COMMENT 'date',
    `time` TIME COMMENT 'time',
    `black_ip` VARCHAR (128) COMMENT 'black_ip',
    PRIMARY KEY (`event_id`)
) ENGINE = INNODB DEFAULT CHARSET = utf8;

CREATE TABLE IF NOT EXISTS `respond_log` (
    `event_id` VARCHAR (128) COMMENT 'event_id',
    `date` DATE COMMENT 'date',
    `time` TIME COMMENT 'time',
    `event` VARCHAR (128) COMMENT 'event',
    `black_ip` VARCHAR (128) COMMENT 'black_ip',
    `counteratack_ip` VARCHAR (128) COMMENT 'counteratack_ip',
    PRIMARY KEY (`event_id`)
) ENGINE = INNODB DEFAULT CHARSET = utf8;