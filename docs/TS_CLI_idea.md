# Calimero Hello App Frontend

This is a TypeScript client application that interacts with a Calimero node. It demonstrates the authentication flow and basic API interactions using the Calimero Client SDK.

## Features

- 🔐 Authentication with Calimero Node
- 📋 List Applications
- 🔄 Token Management
- 🌐 API Client Setup

## Prerequisites

- Node.js (v16 or higher)
- pnpm
- A running Calimero node (default: localhost:2428)
- Application ID from your Calimero node

## Setup

1. Install dependencies:

```bash
pnpm install
```

2. Configure environment variables:

```bash
cp .env.example .env
```

Edit `.env` with your values:

```env
NODE_URL=http://localhost:2428
APPLICATION_ID=your-application-id
```

## Development

Start the development server:

```bash
pnpm dev
```

Build the application:

```bash
pnpm build
```

Run the built application:

```bash
pnpm start
```

## Project Structure

```
frontend/
├── src/
│   ├── index.ts        # Main entry point
│   ├── auth/
│   │   ├── index.ts    # Auth exports
│   │   └── client.ts   # Auth client setup
│   └── api/
│       ├── index.ts    # API exports
│       └── client.ts   # API client setup
├── package.json
└── tsconfig.json
```

## Authentication Flow

1. Client initializes with node URL and application ID
2. Auth client handles:
   - Challenge request
   - Signature generation
   - JWT token management
3. API client uses auth tokens for protected endpoints

## API Methods

Currently implemented:

- `getApplications()`: List all applications

Planned:

- Application details
- Context management
- User management

## Environment Variables

| Variable       | Description         | Default               |
| -------------- | ------------------- | --------------------- |
| NODE_URL       | Calimero node URL   | http://localhost:2428 |
| APPLICATION_ID | Your application ID | Required              |

## Contributing

1. Fork the repository
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request

## License

[Add your license here]
