# API Documentation

---

## Table of contents

* [1. Setup Instructions](#1-setup-instructions)
  
  * [1.1 Linux](#11-linux)
  
  * [1.2 Windows](#12-windows)

* [2. General](#2-general)

* [3. User operations](#3-user-operations)
  
  * [3.1 Login](#31-login)
  
  * [3.2 Logout](#32-logout)
  
  * [3.3 Logged](#33-logged)
  
  * [3.4 Chpwd](#34-chpwd)
  
  * [3.5 Add](#35-add)
  
  * [3.6 Check](#36-check)

* [4.  Data operations](#4-data-operations)
  
  * [4.1 YT_DL](#41-yt_dl)
  
  * [4.2 Medialist](#42-medialist)
  
  * [4.3 Download](#43-download)
  
  * [4.4 Convert](#44-convert)

---

## 1. Setup instructions

### 1.1 Linux

Install required tools e.g:

```shell
yay -S rustup mariadb yt-dlp ffmpeg
```

Setup the rustup toolchain:

```shell
rustup toolchain install nightly
```

Clone and cd into git tree:

```
git clone https://github.com/HellBjoern/mediaportal_api && cd mediaportal_api
```

Setup mariadb:

```shell
sudo mariadb-install-db --user=mysql --basedir=/usr --datadir=/var/lib/mysql
```

Switch to superuser:

```bash
su root
```

Change maximum packet size for mariadb:

```bash
echo "[mysqld]
max_allowed_packet=1G" >> /etc/my.cnf
```

Exit root:

```bash
exit
```

Start mariadb:

```shell
systemctl start mariadb
```

Connect to mariadb:

```shell
sudo mysql -u root -p
```

Add SQL user:

```sql
CREATE USER 'username'@'localhost' IDENTIFIED BY 'password';
```

Grant privileges to user:

```sql
GRANT ALL PRIVILEGES ON *.* TO 'username'@'localhost';
```

Create database:

```sql
CREATE DATABASE db_name;
```

Use new Database:

```sql
USE db_name;
```

Import tables:

```sql
SOURCE mediaportal.sql;
```

Exit mariadb:

```sql
QUIT;
```

Edit api.toml config file to match needs:

```shell
vim api.toml
```

Build:

```shell
cargo build
```

Run:

```shell
cargo run
```

Run with logging enabled:

```bash
RUST_LOG=info cargo run
```

### 1.2 Windows

- [ ] Not coming anytime soon

---

## 2. General

On Success Code is 200

On Error / Failure / Other Code is 400

## 3. User operations

## 3.1 Login

* Checks credentials and on success returns user + uid

* *ip*:*port*/user/login

---

#### 3.11 Request

Example Data:

```json
{
    "username": "username",
    "password": "password"
}
```

---

#### 3.12 Response

Success:

```json
{
    "id": 1,
    "username": "username"
}
```

Failure:

* Sent invalid Json

```
Json deserialize error: ...
```

* Wrong Username / Password

```json
{
    "message": "Wrong Credentials!"
}
```

* Database Error

```json
{
    "message": "Database error message"
}
```

## 3.2 Logout

* Logs user out

* *ip*:*port*/user/logout

---

#### 3.21 Request

Example Data:

```json
{
    "username": "username"
}
```

---

#### 3.22 Response

Success:

```json
{
    "message": "Successfully logged out!"
}
```

Failure:

- Sent invalid Json

```
Json deserialize error: ...
```

* Wrong Username

```json
{
    "message": "User does not exist!"
}
```

- Database Error

```json
{
    "message": "Database error message"
}
```

## 3.3 Logged

- Checks if user is logged in

- *ip*:*port*/user/logged

---

#### 3.31 Request

Example Data:

```json
{
    "username": "username"
}
```

---

#### 3.32 Response

Success:

```json
{
    "logged": true/false
}
```

Failure:

- Sent invalid Json

```
Json deserialize error: ...
```

- Wrong Username

```json
{
    "message": "User does not exist!"
}
```

- Database Error

```json
{
    "message": "Database error message"
}
```

## 3.4 Chpwd

* Changes the password for a user

* *ip*:*port*/user/chpwd

---

#### 3.41 Request

Example Data:

```json
{
    "username": "username",
    "oldpwd": "old_password",
    "newpwd": "new_password"
}
```

---

#### 3.42 Response

Success:

```json
{
    "message": "Changed password successfully!"
}
```

Failure:

- Sent invalid Json

```
Json deserialize error: ...
```

- Username does not exist

```json
{
    "message": "User does not exist!"
}
```

* Wrong old password

```json
{
    "message": "Wrong Credentials!"
}
```

- Database Error

```json
{
    "message": "Database error message"
}
```

## 3.5 Add

- Inserts user into Database

- *ip*:*port*/user/add

---

#### 3.51 Request

Example Data:

```json
{
    "username": "username",
    "email": "email@example.com",
    "password": "sha256"
}
```

---

#### 3.52 Response

Success:

```json
{
    "id": uid,
    "username": "username"
}
```

Failure:

- Sent invalid Json

```
Json deserialize error: ...
```

- Username taken

```json
{
    "message": "User already exists!"
}
```

- Email taken

```json
{
    "message": "Email already taken!"
}
```

- Database Error

```json
{
    "message": "Database error message"
}
```

## 3.6 Check

- Checks if user exists in DB (ONLY FOR TESTING)(SECURITY RISK)

- *ip*:*port*/user/check

---

#### 3.61 Request

Example Data:

```json
{
    "username": "username"
}
```

---

#### 3.62 Response

Success:

```json
{
    "message": true/false
}
```

Failure:

- Sent invalid Json

```
Json deserialize error: ...
```

- Database Error

```json
{
    "message": "Database error message"
}
```

---

## 4. Data operations

## 4.1 YT_DL

* Downloads specified youtube uri in specific format and stores it on DB

* *ip*:*port*/data/yt_dl

---

#### 4.11 Request

Example Data:

```json
{
    "uid": uid,
    "uri": "youtube-uri",
    "format": 1/2/3
}
```

(format can be either 1 (audio only), 2 (video only) or 3 (both))

---

#### 4.12 Response

Success:

```json
{
    "filename": videoname,
    "message": "Successfully downloaded {uri}",
    "mid": mid
}
```

Failure:

- Sent invalid Json

```
Json deserialize error: ...
```

- Database Error

```json
{
    "message": "Database error message"
}
```

- User not logged in

```json
{
    "message": "User is not logged in"
}
```

- Uid does not exist

```json
{
    "message": "User does not exist!"
}
```

* File on server failed to be read

```json
{
    "message": "Failed reading file; Reason: {reason for error}"
}
```

* Supplied format index was invalid

```json
{
    "message": "Supplied invalid format"
}
```

* Supplied YT uri is invalid / network error

```json
{
    "message": "Failed to download video; Verify URL"
}
```

## 4.2 Medialist

- Returns all media linked to a user

- *ip*:*port*/data/medialist

---

#### 4.21 Request

Example Data:

```json
{
    "uid": uid
}
```

---

#### 4.22 Response

Success:

```json
{
    "0": {
        "mformat": 1/2/3,
        "mid": mid,
        "mname": "Video name.mp3/4"
    },
    "1": {
        "mformat": ...
    }
}
```

If no files are present, return will be:

```json
{}
```

Failure:

- Sent invalid Json

```
Json deserialize error: ...
```

- Database Error

```json
{
    "message": "Database error message"
}
```

- Uid does not exist

```json
{
    "message": "User does not exist!"
}
```

## 4.3 Download

- Responds with requested media as binary/octet-stream

- *ip*:*port*/data/download

---

#### 4.31 Request

Example Data:

```json
{
    "uid": uid,
    "mid": mid
}
```

---

#### 4.32 Response

Success:

Sends the media as octet-stream;

Failure:

- Sent invalid Json

```
Json deserialize error: ...
```

- Database Error

```json
{
    "message": "Database error message"
}
```

- Uid does not exist

```json
{
    "message": "User does not exist!"
}
```

* Mid does not exist / user does not own mid

```json
{
    "message": "Invalid mid!"
}
```

* User not logged in

```json
{
    "message": "User is not logged in!"
}
```

## 4.4 Convert

- Converts supplied media to format specified and saves it to DB

- *ip*:*port*/data/convert

---

#### 4.41 Request

Example Data:

```
{
    "uid": uid,
    "file": file,
    "format": 1/2/3
}
```

Data expected is multipart/form

(format can be either 1 (audio only), 2 (video only) or 3 (both))

---

#### 4.42 Response

Success:

```json
{
    "filename": videoname,
    "message": "Successfully converted to {new filename}",
    "mid": mid
}
```

Failure:

- Sent invalid Json

```
Json deserialize error: ...
```

- Database Error

```json
{
    "message": "Database error message"
}
```

- User not logged in

```json
{
    "message": "User is not logged in"
}
```

- Uid does not exist

```json
{
    "message": "User does not exist!"
}
```

- File on server failed to be read

```json
{
    "message": "Failed reading file; Reason: {reason for error}"
}
```

- Conversion failed

```json
{
    "message": "Failed to convert media; Check format / content!"
}
```

- Supplied format index was invalid

```json
{
    "message": "Supplied invalid format"
}
```
