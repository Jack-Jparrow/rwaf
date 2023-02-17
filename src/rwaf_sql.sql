/**
 * @Author       : 白银
 * @Date         : 2023-02-15 17:14:14
 * @LastEditors  : 白银
 * @LastEditTime : 2023-02-17 19:28:50
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

DROP TRIGGER IF EXISTS `backup_log_id_noupdate`;
SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='';
DELIMITER //
CREATE TRIGGER `backup_log_id_noupdate` BEFORE UPDATE ON `backup_log` FOR EACH ROW BEGIN
    set new.event_id  = old.event_id;
END//
DELIMITER ;
SET SQL_MODE=@OLD_SQL_MODE;

DROP TABLE IF EXISTS `system_monitor`;
CREATE TABLE IF NOT EXISTS `system_monitor` (
    `event_id` VARCHAR (128) COMMENT 'event_id',
    `date` DATE COMMENT 'date',
    `time` TIME COMMENT 'time',
    `event` VARCHAR (128) COMMENT 'event',
    `cpu` VARCHAR (128) COMMENT 'cpu',
    `mem` VARCHAR (128) COMMENT 'mem',
    `disk` VARCHAR (128) COMMENT 'disk',
    `net_receive` VARCHAR (128) COMMENT 'net_receive',
    `net_send` VARCHAR (128) COMMENT 'net_send',
    PRIMARY KEY (`event_id`)
) ENGINE = INNODB DEFAULT CHARSET = utf8;

DROP TRIGGER IF EXISTS `system_monitor_id_noupdate`;
SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='';
DELIMITER //
CREATE TRIGGER `system_monitor_id_noupdate` BEFORE UPDATE ON `system_monitor` FOR EACH ROW BEGIN
    set new.event_id  = old.event_id;
END//
DELIMITER ;
SET SQL_MODE=@OLD_SQL_MODE;

CREATE TABLE IF NOT EXISTS `webshell_log` (
    `event_id` VARCHAR (128) COMMENT 'event_id',
    `date` DATE COMMENT 'date',
    `time` TIME COMMENT 'time',
    `event` VARCHAR (128) COMMENT 'event',
    `if_webshell` VARCHAR (128) COMMENT 'if_webshell',
    `if_send_email` VARCHAR (128) COMMENT 'if_send_email',
    PRIMARY KEY (`event_id`)
) ENGINE = INNODB DEFAULT CHARSET = utf8;

DROP TRIGGER IF EXISTS `webshell_log_id_noupdate`;
SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='';
DELIMITER //
CREATE TRIGGER `webshell_log_id_noupdate` BEFORE UPDATE ON `webshell_log` FOR EACH ROW BEGIN
    set new.event_id  = old.event_id;
END//
DELIMITER ;
SET SQL_MODE=@OLD_SQL_MODE;

CREATE TABLE IF NOT EXISTS `black_ip` (
    `event_id` VARCHAR (128) COMMENT 'event_id',
    `date` DATE COMMENT 'date',
    `time` TIME COMMENT 'time',
    `black_ip` VARCHAR (128) COMMENT 'black_ip',
    PRIMARY KEY (`event_id`)
) ENGINE = INNODB DEFAULT CHARSET = utf8;

DROP TRIGGER IF EXISTS `black_ip_id_noupdate`;
SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='';
DELIMITER //
CREATE TRIGGER `black_ip_id_noupdate` BEFORE UPDATE ON `black_ip` FOR EACH ROW BEGIN
    set new.event_id  = old.event_id;
END//
DELIMITER ;
SET SQL_MODE=@OLD_SQL_MODE;

DROP TABLE IF EXISTS `respond_log`;
CREATE TABLE IF NOT EXISTS `respond_log` (
    `event_id` VARCHAR (128) COMMENT 'event_id',
    `date` DATE COMMENT 'date',
    `time` TIME COMMENT 'time',
    `event` VARCHAR (128) COMMENT 'event',
    `black_ip` VARCHAR (128) COMMENT 'black_ip',
    `if_banned` VARCHAR (128) COMMENT 'if_banned',
    `if_send_email` VARCHAR (128) COMMENT 'if_send_email',
    `counterattack_ip` VARCHAR (128) COMMENT 'counteratack_ip',
    PRIMARY KEY (`event_id`)
) ENGINE = INNODB DEFAULT CHARSET = utf8;

DROP TRIGGER IF EXISTS `respond_log_id_noupdate`;
SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='';
DELIMITER //
CREATE TRIGGER `respond_log_id_noupdate` BEFORE UPDATE ON `respond_log` FOR EACH ROW BEGIN
    set new.event_id  = old.event_id;
END//
DELIMITER ;
SET SQL_MODE=@OLD_SQL_MODE;