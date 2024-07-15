# Retrieve User
Retrieves user data from database

**URL** : `/api/user`

**METHOD** : `POST`

**Auth Required** : Yes

## Success Response
**Code** : `200 OK`

Content

```json
{
    "user": "<User>"
}
```

# Create User
Creates a new user

**URL** : `/api/user`

**METHOD** : `POST`

**Auth Required** : No

Data

```json
{
    "email": "Valid email address",
    "password": "Password of at least 10 characters",
    "confirmPassword": "Confirmation password matching the other password",
    "firstName": "First name of the user",
    "lastName": "Last name of the user"
}
```
Data Example

```json
{
    "email": "JohnDoe@mail.com",
    "password": "Password123",
    "confirmPassword": "Password123",
    "firstName": "John",
    "lastName": "Doe"
}
```

## Success Response
**Code** : `200 OK`

Content

```json
{
    "user": "<User>"
}
```
