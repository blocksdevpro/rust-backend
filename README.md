# 🥗 Calorine – Calorie Tracking & Fitness Backend

A high-performance REST API backend for a calorie tracking and fitness web application, built with Rust.

## ✨ Tech Stack

| Layer          | Technology                                                 |
| -------------- | ---------------------------------------------------------- |
| Web Framework  | [Axum](https://github.com/tokio-rs/axum)                   |
| Database       | PostgreSQL via [SQLx](https://github.com/launchbadge/sqlx) |
| Authentication | Google OAuth 2.0 + JWT                                     |
| Storage        | [Cloudflare R2](https://developers.cloudflare.com/r2/)     |
| AI             | [OpenRouter](https://openrouter.ai/) API                   |

## 🚀 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [PostgreSQL](https://www.postgresql.org/download/) 15+
- [sqlx-cli](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli) for running migrations

```bash
cargo install sqlx-cli --no-default-features --features postgres
```

### Setup

1. **Clone the repository**

   ```bash
   git clone https://github.com/blocksdevpro/rust-backend.git
   cd rust-backend
   ```

2. **Configure environment variables**

   ```bash
   cp .env.example .env
   ```

   Then fill in the values in `.env` (see [Environment Variables](#-environment-variables) below).

3. **Run database migrations**

   ```bash
   sqlx migrate run
   ```

4. **Start the server**

   ```bash
   cargo run
   ```

   The server will start at `http://localhost:8080` by default.

## 🔑 Environment Variables

| Variable               | Description                                            |
| ---------------------- | ------------------------------------------------------ |
| `DATABASE_URL`         | PostgreSQL connection string                           |
| `GOOGLE_CLIENT_ID`     | Google OAuth 2.0 Client ID                             |
| `GOOGLE_CLIENT_SECRET` | Google OAuth 2.0 Client Secret                         |
| `GOOGLE_REDIRECT_URI`  | OAuth callback URL (e.g. `/auth/callback`)             |
| `JWT_SECRET`           | Secret key used to sign JWT tokens                     |
| `JWT_EXPIRATION`       | JWT token expiration in hours                          |
| `OPENAI_API_KEY`       | OpenRouter API key (uses OpenAI-compatible API)        |
| `OPENAI_BASE_URL`      | OpenRouter base URL (`https://openrouter.ai/api/v1`)   |
| `CF_R2_BUCKET`         | Cloudflare R2 bucket name                              |
| `CF_ACCOUNT_ID`        | Cloudflare account ID                                  |
| `CF_ACCESS_KEY`        | Cloudflare R2 access key ID                            |
| `CF_ACCESS_SECRET`     | Cloudflare R2 secret access key                        |
| `RUST_LOG`             | Log level (e.g. `rust_backend=debug,tower_http=debug`) |

## 📁 Project Structure

```
rust-backend/
├── migrations/        # SQLx database migrations
├── src/
│   ├── main.rs        # Entry point, server setup
│   ├── error.rs       # Error handling
│   ├── core/          # Core components
│   ├── modules/       # Modules
│
├── .env.example       # Example environment variables
└── Cargo.toml
```

## 🛠️ Development

Run with auto-reloading using `cargo-watch`:

```bash
cargo install cargo-watch
cargo watch -x run
```

## 📄 License

[MIT](LICENSE)

```

```
