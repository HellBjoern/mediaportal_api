# API Documentation

## Build instructions

### Linux

Install rustup e.g:

```shell
yay -S rustup
```

Setup the rustup toolchain:

```shell
rustup toolchain install nightly
```

Clone and cd into git tree:

```
git clone https://github.com/HellBjoern/mediaportal_api && cd mediaportal_api
```

Build:

```shell
cargo build
```

Run:

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