# API Documentation

## Build instructions

### Linux

install rustup f.e:

```shell
yay -S rustup
```

setup the rustup toolchain:

```shell
rustup toolchain install nightly
```

clone and cd into git tree:

```
git clone https://github.com/HellBjoern/mediaportal_api && cd mediaportal_api
```

build:

```shell
cargo build
```

run:

```shell
cargo run
```

---

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
| 452  | Error while creating DB Pool  |
| 453  | Could not connect to Pool     |
| 454  | Error while inserting into DB |

---

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
| 452  | Error while creating DB Pool |
| 453  | Could not connect to Pool    |
| 454  | User does not exist          |
| 455  | Passwords do not match       |