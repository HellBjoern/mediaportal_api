# API Documentation

---

## Setup instructions

### Linux

Install rustup and mariadb e.g:

```shell
yay -S rustup mariadb yt-dlp
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

### Windows

- [ ] Coming soon

---

## General

On Success Code is 200

On Error / Failure / Other Code is 400



## User operations

## Login

* Checks credentials and on success returns user + uid

* *ip*:*port*/user/login

---

**Request**

Example Data:

```json
{
    "username": "username",
    "password": "password"
}
```

---

**Response**

Success:

```json
{
    "id": 1,
    "username": "username"
}
```

Failure:

* Sent invalid Json

```json
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

## Logout

* Logs user out

* *ip*:*port*/user/logout

---

**Request**

Example Data:

```json
{
    "username": "username"
}
```

---

**Response**

Success:

```json
{
    "message": "Successfully logged out!"
}
```

Failure:

- Sent invalid Json

```json
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

## Logged

- Checks if user is logged in

- *ip*:*port*/user/logged

---

**Request**

Example Data:

```json
{
    "username": "username"
}
```

---

**Response**

Success:

```json
{
    "logged": true/false
}
```

Failure:

- Sent invalid Json

```json
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

## Chpwd

* Changes the password for a user

* *ip*:*port*/user/chpwd

---

**Request**

Example Data:

```json
{
    "username": "username",
    "oldpwd": "old_password",
    "newpwd": "new_password"
}
```

---

**Response**

Success:

```json
{
    "message": "Changed password successfully!"
}
```

Failure:

- Sent invalid Json

```json
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

## Add

- Inserts user into Database

- *ip*:*port*/user/add

---

**Request**

Example Data:

```json
{
    "username": "username",
    "email": "email@example.com",
    "password": "sha256"
}
```

---

**Response**

Success:

```json
{
    "id": uid,
    "username": "username"
}
```

Failure:

- Sent invalid Json

```json
Json deserialize error: ...
```

- Username taken

```json
{
    "message": "User already exists!"
}
```

- Database Error

```json
{
    "message": "Database error message"
}
```

## Check

- Checks if user exists in DB (ONLY FOR TESTING)(SECURITY RISK)

- *ip*:*port*/user/check

---

**Request**

Example Data:

```json
{
    "username": "username"
}
```

---

**Response**

Success:

```json
{
    "message": true/false
}
```

Failure:

- Sent invalid Json

```json
Json deserialize error: ...
```

- Database Error

```json
{
    "message": "Database error message"
}
```

---

## Data operations

## YT_DL

* Downloads specified youtube uri in specific format and stores it on DB

* *ip*:*port*/data/yt_dl

---

**Request**

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

**Response**

Success:

```json
{
    "message": "Successfully downloaded {uri}",
    "mid": mid
}
```

Failure:

- Sent invalid Json

```json
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

* Supplied format was invalid

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

## Medialist

- Returns all media linked to a user

- *ip*:*port*/data/medialist

---

**Request**

Example Data:

```json
{
    "uid": uid,
}
```

---

**Response**

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

**Failure**

- Sent invalid Json

```json
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
