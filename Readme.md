# API Documentation

---

## Setup instructions

### Linux

Install rustup and mariadb e.g:

```shell
yay -S rustup mariadb
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

Edit src/main.rs static SQL to match your username and password for SQL:

```shell
vim src/main.rs
```

Build:

```shell
cargo build
```

Run:

```shell
cargo run
```

### Windows

- [ ] Coming soon

---

## Add

* Desciption:
  
  * Adds a user to the db if possible

* Path:
  
  * /user/add

* Type:
  
  * Post

* Takes:
  
  * Json

* Example Data:

```json
{ "username":"username","email":"email@example.com","password":"passwordhash" }
```

* Statuscodes:

| Code | Meaning                       | Working    |
| ---- | ----------------------------- | ---------- |
| 200  | User added                    | :question: |
| 400  | Bad Request / Malformed Json  | :question: |
| 452  | Error while creating DB Pool  | :question: |
| 453  | Could not connect to Pool     | :question: |
| 454  | Error while inserting into DB | :question: |

---

## Login

* Description:
  
  * Verifies the provided username and password

* Path:
  
  * /user/login

* Type:
  
  * Post

* Takes:
  
  * Json

* Example Data:

```json
{ "username":"username","password":"password" }
```

Statuscodes:

| Code | Meaning                      | Working            |
| ---- | ---------------------------- | ------------------ |
| 200  | Successfully logged in       | :heavy_check_mark: |
| 400  | Bad Request / Malformed Json | :heavy_check_mark: |
| 452  | Error while creating DB Pool | :heavy_check_mark: |
| 453  | Could not connect to Pool    | :question:         |
| 454  | User does not exist          | :heavy_check_mark: |
| 455  | Passwords do not match       | :heavy_check_mark: |

---

## Check

* Description:
  
  * Checks if username is present in db

* Path:
  
  * /user/check

* Type:
  
  * Post

* Takes:
  
  * Json

* Example Data:

```json
{ "username":"username" }
```

* Statuscodes:

| Code | Meaning                      | Working            |
| ---- | ---------------------------- | ------------------ |
| 200  | Username exists              | :heavy_check_mark: |
| 400  | Bad Request / Malformed Json | :heavy_check_mark: |
| 452  | Error while creating DB Pool | :heavy_check_mark: |
| 453  | Could not connect to Pool    | :question:         |
| 454  | Username does not exist      | :heavy_check_mark: |

---

## Logged

* Description:
  
  * Check if user is logged in

* Path:
  
  * /user/logged

* Type:
  
  * Post

* Takes;
  
  * Json

* Example Data:

```json
{ "username":"username" }
```

* Statuscodes:

| Code | Meaning                      | Working            |
| ---- | ---------------------------- | ------------------ |
| 200  | Logged in                    | :heavy_check_mark: |
| 400  | Bad Request / Malformed Json | :heavy_check_mark: |
| 452  | Error while creating DB Pool | :heavy_check_mark: |
| 453  | Could not connect to Pool    | :question:         |
| 454  | Username does not exist      | :heavy_check_mark: |
| 455  | Not logged in                | :heavy_check_mark: |
| 456  | Error during db request      | :question:         |