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

# Update User Profile
Updates the user information.

**URL** : `/api/user`

**METHOD** : `PUT`

**Auth Required** : Yes

Data

```json
{
    "location": "ObjectId of the default location of the user",
    "firstName": "User first name",
    "lastName": "User last name",
    "email": "User email",
}
```
Data Example

```json
{
    "location": "66918408fb9f48a01e5c3797",
    "firstName": "John",
    "lastName": "Doe",
    "email": "john@mail.com"
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

# Update User Profile Photo
**URL** : `/api/user/photo`

**METHOD** : `PUT`

**Auth Required** : Yes

Data

```json
{
    "image": "New profile photo image"
}
```


