# Api Documentation

## General

On Success Code is 200

On Error / Failure / Other Code is 400

 

## Login

* Checks credentials and on success returns user + 

* *ip*:*port*/user/login

---

**Request**

Example Data:

```json
{
    "username":"username",
    "password":"password"
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

## 

## Logout

* Logs user out

* *ip*:*port*/user/logout

---

**Request**

Example Data:

```json
{
    "username"
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
    "username"
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

## Add

- Inserts user into Database

- *ip*:*port*/user/add

---

**Request**

Example Data:

```json
{
    "username":"username",
    "email":"email@example.com",
    "password":"sha256"
}
```

---

**Response**

Success:

```json
{
    "id": uid
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
    "username":"username"
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