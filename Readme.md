# API Documentation



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



## Add

Path: /user/add

Type: Post

Takes: Json

Example Data:

```json
{ "username":"username","email":"email@example.com","password":"passwordhash" }
```

Statuscodes

| Code | Meaning                       |
| ---- | ----------------------------- |
| 200  | Ok                            |
| 400  | Bad Request / Malformed Json  |
| 452  | Error while creating DB Pool  |
| 453  | Could not connect to Pool     |
| 454  | Error while inserting into DB |



## Login

Path: /user/login

Type: Post

Takes: Json

Example Data:

```json
{ "username":"username","password":"password" }
```

Statuscodes

| Code | Meaning                      |
| ---- | ---------------------------- |
| 200  | Ok                           |
| 400  | Bad Request / Malformed Json |
| 452  | Error while creating DB Pool |
| 453  | Could not connect to Pool    |
| 454  | User does not exist          |
| 455  | Passwords do not match       |