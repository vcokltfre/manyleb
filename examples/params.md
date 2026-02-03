# Sample API

**Version:** 1.0.0

This is a sample API specification.

## Objects

## Endpoints

### GET /:id

Get by ID

#### Parameters

| Name | Type | Description |
|------|------|-------------|
| id | string | The ID of the user |

#### Query Parameters

| Name | Type | Description |
|------|------|-------------|
| verbose | ?boolean | Whether to include verbose details |

#### Responses

| Status Code | Type |
|-------------|------|
| 200 | [][User](#user) |

