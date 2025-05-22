# RPRT Backend

This is the backend service for the RPRT (Rust PostgreSQL React TypeScript) template, built with Axum and designed for high performance and reliability.

## Features

- 🚀 Blazing fast API with Axum
- 🗃️ PostgreSQL database integration with SQLx
- 🔐 JWT-based authentication
- 📝 Request/Response validation
- 📊 Structured logging
- ⚙️ Environment-based configuration
- 🧪 Test utilities

## Prerequisites

- Rust (latest stable)
- PostgreSQL 13+
- Cargo (Rust's package manager)

## Getting Started

1. **Clone the repository**
   ```bash
   git clone https://github.com/SHA888/RPRT.git
   cd RPRT/backend
   ```

2. **Set up environment variables**
   ```bash
   cp .env.example .env
   # Edit .env with your configuration
   ```

3. **Install SQLx CLI**
   ```bash
   cargo install sqlx-cli
   ```

4. **Set up the database**
   ```bash
   # Create the database (make sure PostgreSQL is running)
   createdb rprt_db
   
   # Run migrations
   sqlx migrate run
   ```

5. **Run the server**
   ```bash
   cargo run
   ```

   The server will start at `http://localhost:3000`

## Project Structure

```
backend/
├── src/
│   ├── api/           # API routes and handlers
│   │   ├── mod.rs     # API module exports
│   │   └── health.rs  # Health check endpoints
│   ├── config.rs      # Configuration management
│   ├── error.rs       # Custom error types
│   └── main.rs        # Application entry point
├── migrations/        # Database migrations
├── .env.example      # Example environment variables
└── Cargo.toml        # Rust dependencies
```

## API Endpoints

### Health Check

- `GET /health` - Check if the server is running

## Development

### Running Migrations

```bash
# Create a new migration
sqlx migrate add <migration_name>

# Run migrations
sqlx migrate run
```

### Testing

```bash
# Run all tests
cargo test

# Run tests with logging
RUST_LOG=debug cargo test -- --nocapture
```

### Linting

```bash
cargo clippy
```

### Formatting

```bash
cargo fmt
```

## Environment Variables

See `.env.example` for all available environment variables.

## Deployment

### Building for Production

```bash
cargo build --release
```

### Docker

```bash
# Build the Docker image
docker build -t rprt-backend .

# Run the container
docker run -p 3000:3000 --env-file .env rprt-backend
```

## License

MIT
