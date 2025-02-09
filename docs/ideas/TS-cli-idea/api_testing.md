# Admin API Endpoints

## Unprotected Endpoints

1. Health Check (Tested)

```bash
curl http://localhost:2428/admin-api/health
```

2. Get Certificate

```bash
curl http://localhost:2428/admin-api/certificate
```

## Protected Endpoints (Require JWT)

1. List Applications

```bash
curl -X GET http://localhost:2428/admin-api/applications \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -H "Content-Type: application/json"
```

2. Get Specific Application

```bash
curl -X GET http://localhost:2428/admin-api/applications/8YnK2NThDuosRC5MTpGXu1XEFTN5MpeMRWQt9cBGBVey \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -H "Content-Type: application/json"
```
