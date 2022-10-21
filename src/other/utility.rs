use mysql::{params, Pool, PooledConn, prelude::Queryable};

//returns sql pooled conn or error as string to be used for error handling
pub fn get_conn_fn() -> Result<PooledConn, String> {
    let pool = match  Pool::new(crate::SQL) {
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