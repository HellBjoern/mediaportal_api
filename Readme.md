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

| Code | Meaning         |
| ---- | --------------- |
| 200  | Ok              |
| 452  | Error Inserting |

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

| Code | Meaning                 |
| ---- | ----------------------- |
| 200  | Ok                      |
| 452  | Username does not exist |
| 453  | Wrong password          |