# 🚀 RPRT (Rust PostgreSQL React TypeScript) Template

A production-ready, full-stack template for building modern web applications. This monorepo combines the performance and safety of Rust with the flexibility of React and TypeScript, providing a solid foundation for scalable web applications.

## 🏗️ Project Structure

```
RPRT/
├── backend/           # Rust backend service
│   ├── src/           # Source code
│   ├── api/           # API routes and handlers
│   ├── migrations/     # Database migrations
│   └── Cargo.toml     # Rust dependencies
│
├── frontend/         # React frontend
│   ├── src/           # Source code
│   ├── public/        # Static assets
│   └── package.json   # Node.js dependencies
│
├── scripts/          # Development and deployment scripts
│   └── PRD.txt        # Product Requirements Document
│
├── tasks/            # Task management files
│   └── ...
│
├── .gitignore        # Git ignore rules
└── README.md         # This file
```

## 🎯 Features

- **Backend**: High-performance Rust server with Axum
- **Frontend**: Modern React with TypeScript and Vite
- **Database**: PostgreSQL with SQLx for type-safe queries
- **Development**: Pre-configured with essential tools and workflows

## 🚀 Getting Started

### Prerequisites

- Rust (latest stable)
- Node.js (v20+)
- PostgreSQL (v17+ recommended)
- Git

### Quick Start

1. Clone the repository:
   ```bash
   git clone https://github.com/SHA888/RPRT.git
   cd RPRT
   ```

2. Set up the backend:
   ```bash
   cd backend
   cp .env.example .env
   # Edit .env with your database credentials
   cargo run
   ```

3. Set up the frontend:
   ```bash
   cd ../frontend
   npm install
   npm run dev
   ```

## 📚 Documentation

- [Backend Development](./backend/README.md)
- [Frontend Development](./frontend/README.md)
- [Deployment Guide](./docs/DEPLOYMENT.md)
- [API Reference](./docs/API.md)

## 🤝 Contributing

Contributions are welcome! Please read our [Contributing Guide](./CONTRIBUTING.md) for details.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🛠️ Tech Stack

- **Backend**: Rust (Axum for REST APIs, SQLx for database access)
- **Database**: PostgreSQL (relational database)
- **Frontend**: React with TypeScript (Vite for fast builds, Tailwind CSS for styling)
- **Tools**: GitHub Actions for CI/CD, optional Docker support

## 📋 Prerequisites

Before you begin, ensure you have the following installed:

- **Rust**: Stable version (install via [rustup](https://rustup.rs/))
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **Node.js**: v20 or higher (install via [nvm](https://github.com/nvm-sh/nvm) or [official installer](https://nodejs.org/))
- **PostgreSQL**: v17 recommended (install via package manager, e.g., `brew install postgresql` on macOS)
- **Git**: For version control

## 🚀 Setup

### 1. Clone the Repository

```bash
git clone https://github.com/SHA888/RPRT.git
cd RPRT
```

### 2. Backend Setup

```bash
# Navigate to backend directory
cd backend

# Copy environment file
cp .env.example .env

# Edit .env with your database credentials
# DATABASE_URL=postgres://user:password@localhost:5432/rprt_db

# Install SQLx CLI and run migrations
cargo install sqlx-cli
sqlx migrate run

# Start the backend
cargo run
```

The backend will be available at: http://localhost:3000

### 3. Frontend Setup

```bash
# Navigate to frontend directory
cd frontend

# Copy environment file
cp .env.example .env

# Edit .env with your API URL
# VITE_API_URL=http://localhost:3000

# Install dependencies
npm install

# Start the development server
npm run dev
```

The frontend will be available at: http://localhost:5173

### 4. Database Setup

1. Ensure PostgreSQL is running locally
2. Create the database:
   ```bash
   createdb rprt_db
   ```
3. Migrations are applied automatically during backend setup

## 🐳 Optional Docker Setup

For containerized development:

1. Ensure Docker and Docker Compose are installed
2. Run the full stack:
   ```bash
   docker-compose -f docker/docker-compose.yml up --build
   ```

Access the services:
- Backend: http://localhost:3000
- Frontend: http://localhost:5173
- PostgreSQL: localhost:5432

## 🧪 Testing

### Backend Tests

```bash
cd backend
cargo test
```

### Frontend Tests

```bash
cd frontend
npm test
```

## 🚀 Deployment

- **Backend**: Deploy to [Fly.io](https://fly.io/), [Render](https://render.com/), or AWS ECS
- **Frontend**: Deploy to [Vercel](https://vercel.com/), [Netlify](https://www.netlify.com/), or [Cloudflare Pages](https://pages.cloudflare.com/)
- **Database**: Use managed PostgreSQL on [Supabase](https://supabase.com/), [Neon](https://neon.tech/), or AWS RDS

Don't forget to update environment variables in `.env` files for production URLs.

## 🤝 Contributing

We welcome contributions! Here's how to get started:

1. Fork the repository
2. Create a feature branch: `git checkout -b feature-name`
3. Make your changes
4. Commit your changes: `git commit -m "Add feature"`
5. Push to the branch: `git push origin feature-name`
6. Open a pull request

Found a bug or have a suggestion? Please [open an issue](https://github.com/SHA888/RPRT/issues).

## 📄 License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
