# E-commerce API with Rust

A simple e-commerce REST API built with Rust, featuring authentication and CRUD operations for products.

## Features

- User authentication (register/login) with JWT
- Product management (CRUD operations)
- PostgreSQL database with SQLx
- CORS enabled
- Error handling
- Input validation

## Prerequisites

- Rust (latest stable version)
- PostgreSQL
- Docker (optional)

## Setup

1. Clone the repository:
```bash
git clone <repository-url>
cd simple-crud-rust
```

2. Create a `.env` file in the root directory with the following content:
```env
DATABASE_URL=postgresql://postgres:postgres@localhost:5432/ecommerce_db
JWT_SECRET=your_jwt_secret_key
RUST_LOG=debug
```

3. Set up the database:
```bash
# If using Docker
docker run --name postgres -e POSTGRES_PASSWORD=postgres -p 5432:5432 -d postgres

# Create the database
psql -U postgres -c "CREATE DATABASE ecommerce_db"

# Run migrations (using sqlx-cli)
cargo install sqlx-cli
sqlx migrate run
```

4. Build and run the application:
```bash
cargo build
cargo run
```

The server will start at `http://localhost:8080`

## API Endpoints

### Authentication
- `POST /api/auth/register` - Register a new user
- `POST /api/auth/login` - Login and get JWT token

### Products
- `POST /api/products` - Create a new product
- `GET /api/products` - Get all products
- `GET /api/products/{id}` - Get a specific product
- `PATCH /api/products/{id}` - Update a product
- `DELETE /api/products/{id}` - Delete a product

## Request Examples

### Register User
```bash
curl -X POST http://localhost:8080/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com",
    "password": "password123",
    "full_name": "John Doe"
  }'
```

### Login
```bash
curl -X POST http://localhost:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com",
    "password": "password123"
  }'
```

### Create Product
```bash
curl -X POST http://localhost:8080/api/products \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Sample Product",
    "description": "This is a sample product",
    "price": 29.99,
    "stock": 100
  }'
```

## License

MIT 