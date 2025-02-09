# Calimero Node API Testing

## Authentication Flow

The authentication process requires several steps to establish a secure connection with the node:

### 1. Request Challenge

First, get a challenge from the server:

```bash
curl -X POST http://localhost:2428/admin-api/request-challenge \
  -H "Content-Type: application/json" \
  -d '{}'
```

Response:

```json
{
  "data": {
    "nonce": "3VVdNrhk0XyaLaujHpRIGlezYngjrq0oO46LgGMHkRM=",
    "contextId": null,
    "timestamp": 1738114805,
    "nodeSignature": "Zf0sRaDqtJc27i7M/50rJqjvTmibP8UxNrIdNvEqDRowb2c5wtK9o6Mlns0rv9gjXxOC5gObCvAF8mV57zJHDg=="
  }
}
```

### 2. Add Client Key

Using the challenge response, register your client key:

```bash
curl -X POST http://localhost:2428/admin-api/add-client-key \
  -H "Content-Type: application/json" \
  -d '{
    "nonce": "3VVdNrhk0XyaLaujHpRIGlezYngjrq0oO46LgGMHkRM=",
    "signature": "YOUR_SIGNED_NONCE",
    "timestamp": 1738114805
  }'
```

### 3. Generate JWT Token

After registering the client key, get a JWT token:

```bash
curl -X POST http://localhost:2428/admin-api/generate-jwt-token \
  -H "Content-Type: application/json" \
  -d '{
    "nonce": "3VVdNrhk0XyaLaujHpRIGlezYngjrq0oO46LgGMHkRM=",
    "signature": "YOUR_SIGNED_NONCE",
    "timestamp": 1738114805
  }'
```

Expected Response:

```json
{
  "data": {
    "token": "eyJhbGciOiJIUzI1...",
    "expiresIn": 3600
  }
}
```

### 4. Use JWT Token

Once you have the token, use it for protected endpoints:

```bash
curl -X GET http://localhost:2428/admin-api/applications \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -H "Content-Type: application/json"
```

### 5. Refresh Token (if needed)

When the token expires:

```bash
curl -X POST http://localhost:2428/admin-api/refresh-jwt-token \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -H "Content-Type: application/json"
```

## Notes

1. The nonce must be signed with your private key
2. The timestamp must match the one received in the challenge
3. The same signature can be used for both add-client-key and generate-jwt-token steps
4. Tokens expire after 1 hour (3600 seconds)

## Health Check

```bash
curl http://localhost:2428/admin-api/health
```

Response:

```json
{
  "data": {
    "status": "alive"
  }
}
```

## Protected Endpoints (TODO)

- List Applications
- List Contexts
- Query Context
- More to be added...

### Unprotected Endpoints

#### Authentication

- `GET /admin-api/health` - Health check
- `GET /admin-api/certificate` - Get certificate
- `POST /admin-api/request-challenge` - Request challenge
- `POST /admin-api/add-client-key` - Add client key
- `POST /admin-api/refresh-jwt-token` - Refresh JWT token

#### Application Management

- `POST /admin-api/root-key` - Create root key
- `POST /admin-api/install-application` - Install application
- `POST /admin-api/uninstall-application` - Uninstall application
- `GET /admin-api/applications` - List applications
- `GET /admin-api/applications/:app_id` - Get application details

### Get Node Certificate

```bash
curl http://localhost:2428/admin-api/certificate
```

Response:

```
-----BEGIN CERTIFICATE-----
MIIBtTCCAVygAwIBAgIUGBtu3chZkp057/rpJvEZTe4X8TEwCgYIKoZIzj0EAwIw
RzFFMEMGA1UEAww8Q2FsaW1lcm8gc2VsZi1zaWduZWQgY2VydGlmaWNhdGUgZm9y
IGxvY2FsIElQOiAiMTcyLjIwLjEwLjIiMCAXDTc1MDEwMTAwMDAwMFoYDzQwOTYw
MTAxMDAwMDAwWjBHMUUwQwYDVQQDDDxDYWxpbWVybyBzZWxmLXNpZ25lZCBjZXJ0
aWZpY2F0ZSBmb3IgbG9jYWwgSVA6ICIxNzIuMjAuMTAuMiIwWTATBgcqhkjOPQIB
BggqhkjOPQMBBwNCAAT8CRSCwRrjavX7vXBPR0UJXiPa10j0iwAE4HPY4eXO+ED+
Z+cMQFteSoI4ZyN4z3cfouKDpKixfddV/xx392OwoyQwIjAgBgNVHREEGTAXhwSs
FAoChwR/AAABgglsb2NhbGhvc3QwCgYIKoZIzj0EAwIDRwAwRAIgAIPEUQpV96Q4
BOwmYGtbKggPt/TMVIEGQpSFO71myUUCIBPrf+BUwSI0GRpiafzNoa1yrs7Q7cRv
cHFmQiQXdUpR
-----END CERTIFICATE-----
```

This is a self-signed SSL/TLS certificate for the Calimero node. Key details:

1. **Type**: Self-signed certificate (created by the node itself)
2. **Purpose**: Used for secure HTTPS communications
3. **Details**:
   - For local IP: "172.20.10.2"
   - Valid for localhost connections
   - Uses ECDSA (Elliptic Curve Digital Signature Algorithm) for cryptography
   - Has an extremely long validity period (from 1975 to 4096)

This certificate is important for:

- Establishing secure connections to the node
- Verifying the node's identity
- Enabling HTTPS/WSS (WebSocket Secure) communications
