use mysql::{Pool, prelude::Queryable, params};

/*
* Utility functions
*/
//returns
//success: true/false
//fail: 452, 453
pub fn checkname(username: String) -> Result<bool, u16>{
    let pool = match  Pool::new(crate::SQL) {
        Ok(pret) => pret,
        Err(err) => {
            println!("Could not create Pool; Error:\n{:?}", err);
            return Err(452);
        },
    };

    let mut conn = match pool.get_conn() {
        Ok(pooled_con) => pooled_con,
        Err(err) => {
            println!("Connection failed; Error:\n{:?}", err);
            return Err(453);
        },
    };

    let res= match conn.exec_first("SELECT uusername FROM users WHERE uusername =:uname", params! { "uname" => &username}).map(|row: Option<String>| { row }) {
        Ok(ret) => ret,
        Err(_) => None,
    };
    if res.is_none() {
        return Ok(false);
    } else {
        return Ok(true);
    }
}

//returns
//success: true / false
//fail: 452, 453, 454, 455
pub fn logged(username: String) -> Result<bool, u16>{
    match checkname(username.clone()) {
        Ok(res) => {
            if !res {
                return Err(454);
            }
        },
        Err(code) => {
            println!("Checkname failed! Code was {}", code);
            return Err(code);
        }
    };

    let pool = match  Pool::new(crate::SQL) {
        Ok(pret) => pret,
        Err(err) => {
            println!("Could not create Pool; Error:\n{:?}", err);
            return Err(452);
        },
    };

    let mut conn = match pool.get_conn() {
        Ok(pooled_con) => pooled_con,
        Err(err) => {
            println!("Connection failed; Error:\n{:?}", err);
            return Err(453);
        },
    };

    match conn.exec_first("SELECT ulogged FROM users WHERE uusername =:uname", params! { "uname" => username }).map(|row: Option<bool>| { row.unwrap() }) {
        Ok(ret) => return Ok(ret),
        Err(_) => return Err(455),
    };
}