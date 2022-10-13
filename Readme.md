# API Documentation

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