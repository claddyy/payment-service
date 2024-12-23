### Payment Service Backend 

A payment service implementation built with Rust, featuring user management, transaction processing, and account balance tracking. This service provides a RESTful API using the Actix web framework with PostgreSQL as the database backend.

### Core Functionality
- User Management
  - Registration and authentication
  - JWT-based authorization
  - Secure password handling with bcrypt
- Transaction Management
  - Create and process transactions
  - Transaction history tracking
  - Per-user transaction listing
- Account Management
  - Multiple accounts per user
  - Account creation and management

### Technical Features
- Built with `actix-web` for high performance
- Asynchronous operations using Rust's async/await
- Rate limiting with `actix-governor`
- Request logging and tracing
- CORS support
- Structured API error responses

### Project Structure

```
src/
├── app_state.rs       # Application state management
├── constants.rs       # Global constants
├── features/
│   ├── accounts/      # Account management
│   ├── transactions/  # Transaction processing
│   ├── user/          # User profile management
│   └── healthcheck/   # Service health check
├── middlewares/       # Middleware for JWT and CORS
├── routes.rs          # API route configuration
├── types.rs          # Common type definitions
├── util.rs           # Utility functions and error handling
└── main.rs           # Application entry point
```

### Prerequisites

- Rust : Install using rustup and/or checkout [documentation](https://www.rust-lang.org/tools/install)
- PostgreSQL : Download from [here](https://www.postgresql.org/download/)
- Docker (optional) : Download from [here](https://www.docker.com/products/docker-desktop/)
- sea-orm-cli : Install using `cargo install sea-orm-cli@1.1.0`

### Getting Started

1. Clone the repository:
   ```bash
   git clone https://github.com/claddyy/dodo-assignment.git
   cd dodo-assignment
   ```

2. Set up the database:
   ```bash
   # PostgreSQL connection details should be configured in your .env
   # Set you database url in .env.example
   mv .env.example .env
   cd db/
   sea-orm-cli migrate fresh #This migrates the database to schema.
   
   ```
> To install sea-orm-cli, simply run `cargo install sea-orm-cli@1.1.0`
4. Run the server
   ```bash
   cd ..
   cd http/
   cargo run
   ```

The service will start on `localhost:3000` by default.

**Installation using Docker**

The setup handles everything automatically, including the database setup and migrations.

```
git clone https://github.com/claddyy/dodo-assignment.git
cd dodo-assignment/
docker-compose up --build
```


### API Documentation

The service exposes the following API endpoints:

**User Management**
- `POST /api/user/register` - Register a new user
- `POST /api/user/login` - Authenticate a user

**Account Management**
- `POST /api/account/create` - Create a new account
- `GET /api/account/{account_id}` - Get account details
- `GET /api/account/list/acc` - List all user accounts
- `GET /api/account/{account_id}/balance` - Get account balance

**Transaction Management**
- `POST /api/transaction/create` - Create a new transaction
- `GET /api/transaction/{transaction_id}` - Get transaction details
- `GET /api/transaction/usertx/tx` - List user transactions

### Documentation
This repo support `rustdocs` and documentation can be generated with:
```bash
cargo doc
```

### Code Formatting
Format your code with:
```bash
cargo fmt
```
