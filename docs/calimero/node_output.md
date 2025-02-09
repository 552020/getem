# Calimero Node Output Analysis

When running `merod --node-name node1 run`, we get several important pieces of information:

## 1. Node Identity

```
Peer ID: 12D3KooWDMCcS3uTqF54rP5pvUEFAvDRESkdjASriHz6saP2nbnE
```

## 2. Server Endpoints

### JSON-RPC Server

- `/ip4/127.0.0.1/tcp/2428/http{/jsonrpc}`
- `/ip6/::1/tcp/2428/http{/jsonrpc}`

### WebSocket Server

- `/ip4/127.0.0.1/tcp/2428/ws{/ws}`
- `/ip6/::1/tcp/2428/ws{/ws}`

### Admin API & Dashboard

- API: `/ip4/127.0.0.1/tcp/2428/http{/admin-api}`
- Dashboard: `/ip4/127.0.0.1/tcp/2428/http{/admin-dashboard}`

## 3. P2P Network Listeners

### TCP Listeners

- `/ip4/127.0.0.1/tcp/2528/p2p/[PEER_ID]`
- `/ip4/172.20.10.2/tcp/2528/p2p/[PEER_ID]`

### QUIC Listeners

- `/ip4/127.0.0.1/udp/2528/quic-v1/p2p/[PEER_ID]`
- `/ip4/172.20.10.2/udp/2528/quic-v1/p2p/[PEER_ID]`

## 4. Relay Connection

Connected to relay at:

- `/ip4/18.156.18.6/tcp/4001/p2p/12D3KooWMgoF9xzyeKJHtRvrYwdomheRbHPELagWZwTLmXb6bCVC`

## Important Notes

1. Node is properly initialized and running
2. All required services (RPC, WebSocket, Admin) are active
3. Both local and public network interfaces are configured
4. SSL certificate needs to be installed (see documentation link)
