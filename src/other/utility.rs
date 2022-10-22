use std::path::Path;
use log::{warn, info};
use mysql::{params, Pool, PooledConn, prelude::Queryable};

use crate::{other::structs::Config, SQL};

//returns sql pooled conn or error as string to be used for error handling
pub fn get_conn_fn() -> Result<PooledConn, String> {
    let pool = match  Pool::new(SQL.as_str()) {
        Ok(pool) => pool,
        Err(err) => return Err(format!("Connection failed: {:?}", err)),
    };

    match pool.get_conn() {
        Ok(pooled_con) => return Ok(pooled_con),
        Err(err) => return Err(format!("Connection failed: {:?}", err)),
    };
}

//checks if username exists in database; returns true / false or string containing error on failure
pub fn checkname_fn(username: String) -> Result<bool, String>{
    let mut conn = match get_conn_fn() {
        Ok(conn) => conn,
        Err(err) => return Err(err),
    };

    match conn.exec_first("SELECT uusername FROM users WHERE uusername =:uname", params! { "uname" => &username}).map(|row: Option<String>| { row }) {
        Ok(res) => {
            if res.is_none() {
                return Ok(false);
            } else {
                return Ok(true);
            }
        },
        Err(err) => return Err(err.to_string()),
    };
}

//returns if user is marked as logged in database; returns true / false or string containing error on failure
pub fn logged_fn(username: String) -> Result<bool, String>{
    match checkname_fn(username.clone()) {
        Ok(exists) => {
            if !exists {
                return Err("User does not exist!".to_string());
            }
        },
        Err(err) => return Err(err),
    };

    let mut conn = match get_conn_fn() {
        Ok(conn) => conn,
        Err(err) => return Err(err),
    };

    match conn.exec_first("SELECT ulogged FROM users WHERE uusername =:uname", params! { "uname" => username }).map(|row: Option<bool>| { row.unwrap() }) {
        Ok(ret) => return Ok(ret),
        Err(err) => return Err(err.to_string()),
    };
}

pub fn get_conf() -> Config {
    match std::fs::read_to_string(crate::CONFPATH) {
        Ok(content) => {
            match toml::from_str(&content) {
                Ok(conf) => {
                    info!("successfully loaded config file");
                    return conf
                },
                Err(err) => warn!("failed to parse config file, falling back to default; reason: {err}, tried loading {}", Path::new(crate::CONFPATH).display()),
            }
        },
        Err(err) => warn!("failed to load config file, falling back to default; reason: {err}, tried loading {}", Path::new(crate::CONFPATH).display()),
    };
    return Config {
        ip: "0.0.0.0".to_string(),
        port: 8080,
        sqladd: "127.0.0.1".to_string(),
        sqlusr: "user".to_string(),
        sqlpwd: "password".to_string(),
        sqlprt: 3306,
        sqldab: "mediaportal".to_string(),
    }
}