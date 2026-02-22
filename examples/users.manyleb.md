# Sample Schema

**Version:** 1.0.0

This is a sample schema for demonstration purposes.

## Objects

### User

| Property | Type | Description |
|----------|------|-------------|
| id | string | Unique identifier for the user. |
| username | string | Username of the user. |
| email | string | Email address of the user. |
| created_at | string | ISO8601 Timestamp of when the user was created. |

### RegisterUserBody

| Property | Type | Description |
|----------|------|-------------|
| username | string | Desired username for the new user. |
| email | string | Email address of the new user. |
| password | string | Password for the new user account. |

### LoginUserBody

| Property | Type | Description |
|----------|------|-------------|
| username | string | Username of the user. |
| password | string | Password of the user. |

### UserTokenResponse

| Property | Type | Description |
|----------|------|-------------|
| token | string | JWT authentication token. |
| user | [User](#user) | Details of the registered user. |

## Endpoints

### POST /api/v1/users/register

Register a new user.

#### Responses

| Status Code | Type |
|-------------|------|
| 201 | [UserTokenResponse](#usertokenresponse) |
| 400 | No Content |

### POST /api/v1/users/login

Authenticate a user and obtain a JWT token.

#### Responses

| Status Code | Type |
|-------------|------|
| 200 | [UserTokenResponse](#usertokenresponse) |
| 401 | No Content |

### GET /api/v1/users/{id}

Retrieve user details by user ID.

#### Responses

| Status Code | Type |
|-------------|------|
| 200 | [User](#user) |
| 404 | No Content |

