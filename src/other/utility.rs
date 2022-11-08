use std::{path::Path, fs::{File, self}, io::Read};
use checked_command::CheckedCommand;
use log::{warn, info};
use mysql::{params, Pool, PooledConn, prelude::Queryable};
use rand::distributions::{Alphanumeric, DistString};

use crate::{other::{structs::Config}, SQL, CONFIG};

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

//checks if email exists in database; returns true / false or string containing error on failure
pub fn checkmail_fn(email: String) -> Result<bool, String>{
    let mut conn = match get_conn_fn() {
        Ok(conn) => conn,
        Err(err) => return Err(err),
    };

    match conn.exec_first("SELECT uemail FROM users WHERE uemail =:uemail", params! { "uemail" => &email}).map(|row: Option<String>| { row }) {
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

//checks if username exists in database; returns true / false or string containing error on failure
pub fn checkuid_fn(uid: i32) -> Result<bool, String>{
    let mut conn = match get_conn_fn() {
        Ok(conn) => conn,
        Err(err) => return Err(err),
    };

    match conn.exec_first("SELECT uid FROM users WHERE uid =:uid", params! { "uid" => &uid}).map(|row: Option<i32>| { row }) {
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
pub fn logged_uname_fn(username: String) -> Result<bool, String>{
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

//returns if user is marked as logged in database; returns true / false or string containing error on failure
pub fn logged_uid_fn(uid: i32) -> Result<bool, String>{
    match checkuid_fn(uid.clone()) {
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

    match conn.exec_first("SELECT ulogged FROM users WHERE uid =:uid", params! { "uid" => uid }).map(|row: Option<bool>| { row.unwrap() }) {
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
        tmppath: "./tmp/".to_string(),
    }
}

//downloads a yt video to folder specified in config; first string is message, second filename for database saving
pub fn yt_dl(uri: String, format: i32) -> Result<(Vec<String>, String), String> {
    let args = match format {
        //audio only
        1 => {
            info!("downloading audio only");
            vec!["-x", "--audio-format", "mp3", "--audio-quality", "0"]
        },
        //video only
        2 => {
            info!("downloading video only");
            vec![r#"-f "bv[ext=mp4]/best[ext=mp4]/best"#]
        },
        //audio + video
        3 => {
            info!("downloading audio + video");
            vec![r#"-f "bv[ext=mp4]+ba[ext=m4a]/best[ext=mp4]/best"#]
        },
        //invalid format
        _ => {
            return Err("Supplied invalid format".to_string());
        }
    };

    let fdlname = match create_tmp_name() {
        Ok(ok) => ok,
        Err(err) => return Err(format!("Failed to create tmp name {}", err)),
    };

    let saveloc = format!("-o{}/{}/%(title)s.%(ext)s", CONFIG.tmppath, fdlname.clone());
    let output = CheckedCommand::new("yt-dlp").args(&args).arg(&uri).arg(saveloc).spawn().expect("failed to execute process").wait_with_output();
    match &output {
        Ok(_) => {
            let paths = fs::read_dir(format!("{}/{}", CONFIG.tmppath, fdlname.clone())).unwrap();
            let mut filenames: Vec<String> = Vec::new();
            for path in paths {
                filenames.push(path.unwrap().path().display().to_string());
            }
            return Ok((filenames, fdlname.clone()))
        },
        Err(_) => return Err("Failed to download video; Verify URL".to_string()),
    };
}

pub fn read_to_vec(path: String) -> Result<Vec<u8>, String> {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(err) => return Err(format!("failed to open file; reason: {:?}", err)),
    };

    let mut data = Vec::new();
    match file.read_to_end(&mut data) {
        Ok(_) => {},
        Err(err) => return Err(format!("failed to read file; reason: {}", err)),
    };

    return Ok(data);
}

pub fn create_tmp_name() -> Result<String, String> {
    match fs::create_dir_all(CONFIG.tmppath.clone()) {
        Ok(_) => {},
        Err(err) => return Err(format!("Failed creating tmp folder; Reason: {}", err)), 
    };

    let exists = match fs::read_dir(format!("{}/", CONFIG.tmppath.clone())) {
        Ok(ok) => ok,
        Err(err) => return Err(format!("Failed to read tmp dir; Reason: {}", err)),
    };
    let mut taken: Vec<String> = Vec::new();
    for f in exists {
        taken.push(f.unwrap().path().file_name().unwrap().to_str().unwrap().to_string());
    }

    let mut rand: String;
    loop {
        rand = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
        if !taken.contains(&rand) {
            break;
        }
    }
    return Ok(rand);
}


pub fn ffmpeg(format: i32, infile: String, mut fname: String) -> Result<String, String> {

    //replaces everything after last . with "" to remove file extensions
    let offset = fname.find(".").unwrap_or(fname.len());
    fname.replace_range(offset.., "");
    
    //input file as argument for ffmpeg
    let input = vec!["-i", &infile];

    //random path in tmp
    let rndpath = match create_tmp_name() {
        Ok(ok) => format!("{}/{}", CONFIG.tmppath, ok),
        Err(err) => return Err(format!("Failed to create tmp name {}", err)),
    };
    match fs::create_dir_all(rndpath.clone()) {
        Ok(_) => {},
        Err(err) => return Err(format!("Failed creating tmp folder; Reason: {}", err)), 
    };
    //thisll be returned in order to find converted output file
    let outfile: String;
    let outargs = match format {
        //audio only
        1 => {
            info!("converting to mp3");
            outfile = format!("{}/{}.mp3", rndpath, fname);
            vec![outfile.clone()]
        },
        //video only
        2 => {
            info!("muting video");
            outfile = format!("{}/{}.mp4", rndpath, fname);
            vec!["-an".to_string(), outfile.clone()]
        },
        //audio + video
        3 => {
            info!("converting to mp4");
            outfile = format!("{}/{}.mp4", rndpath, fname);
            vec![outfile.clone()]
        },
        //invalid format
        _ => {
            return Err("Supplied invalid format".to_string());
        }
    };

    let output = CheckedCommand::new("ffmpeg").args(input).args(outargs).spawn().expect("failed to execute process").wait_with_output();
    match &output {
        Ok(_) => {
            return Ok(outfile);
        },
        Err(_) => {
            match fs::remove_dir_all(Path::new(&rndpath)) {
                Ok(_) => {},
                Err(err) => warn!("failed deleting file; reason: {}", err),
            };
            return Err("Failed to convert media; Check format / content!".to_string())
        },
    };
}