# Sample API

**Version:** 1.0.0

This is a sample API specification.

## Objects

### User

| Property | Type | Description |
|----------|------|-------------|
| id | string | Unique identifier for the user |
| name | string | Name of the user |
| age | ?integer | Age of the user |

## Endpoints

### GET /users

Retrieve a list of users

#### Responses

| Status Code | Type |
|-------------|------|
| 200 | [][User](#user) |

### GET /users/:id

Retrieve a specific user by ID

#### Responses

| Status Code | Type |
|-------------|------|
| 200 | [User](#user) |
| 404 | No Content |

