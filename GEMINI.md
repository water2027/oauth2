# Project Overview: oauth2

`oauth2` is a Rust-based OAuth2 and Identity service built following Domain-Driven Design (DDD) principles. It manages user identities, authentication, and standard OAuth2 authorization flows, providing a clean separation of concerns and a robust architectural approach.

## Tech Stack
- **Language:** Rust (Edition 2024)
- **Web Framework:** Axum (with Tower for middleware)
- **Runtime:** Tokio (Async/Await)
- **Database:** PostgreSQL (via SQLx)
- **Cache:** Redis
- **Serialization:** Serde / Serde JSON
- **Error Handling:** thiserror
- **Password Hashing:** Argon2
- **Email Delivery:** Lettre (with QQ SMTP support)
- **Other:** Chrono (Time), UUID, async-trait

## Architecture & Structure
The project strictly adheres to DDD and is organized into distinct contexts within the `src/` directory:

- **`src/context/`**: Contains the core domain logic.
  - **`identity/`**: Manages user identities, including registration, login, sessions, password hashing, and email verification codes.
    - `application/`: Application services (orchestration, e.g., `AuthAppService`).
    - `entity/`: Domain entities (User, Session, ValidationCode).
    - `value_object/`: Immutable domain values ensuring type safety (Email, UserID, HashedPassword).
    - `service/`: Domain services.
    - `repository/`: Repository interfaces.
    - `infrastructure/`: Implementations of repositories (PostgreSQL via SQLx, Redis).
  - **`oauth/`**: Handles the OAuth2 flow and standard endpoints (Tokens, Authorization, OpenID Connect discovery).
- **`src/presentation/`**: The interface layer.
  - **`http/`**: HTTP API endpoints built with Axum, including routing and authentication middleware.
- **`src/shared/`**: Common utilities, shared error definitions, and global infrastructure configurations (PostgreSQL pools, Redis clients, Email dispatchers).

## Features
- **User Identity:** Registration, login, password modification, profile management, and email verification.
- **OAuth2 Flow:**
  - Client registration (generating `client_id`, `client_secret`, and `redirect_uri`).
  - Authorization code flow (`GET /authorize`, `POST /authorize`).
  - Token exchange and refresh (`POST /token`).
  - Token revocation (`POST /revoke`).
  - OpenID Connect discovery (`/.well-known/openid-configuration`) and JWKS (`/jwks`).

## Building and Running
Ensure you have a `.env` file configured based on `.env.example`. It requires `DATABASE_URL`, `REDIS_URL`, and SMTP credentials (`SMTP_USERNAME`, `SMTP_PASSWORD`).

- **Build:** `cargo build`
- **Run:** `cargo run`
- **Test:** `cargo test`
- **Database Migrations:** `sqlx migrate run` (Requires `sqlx-cli` and PostgreSQL running)

## Development Conventions
- **DDD Compliance:** Keep business logic encapsulated within the `context/` domain layer. Use application services to orchestrate flows and infrastructure layers to handle external I/O.
- **Type Safety via Value Objects:** Heavily utilize Value Objects (found in `src/context/*/value_object/`) to enforce business rules and validation at the type level.
- **Compile-Time Queries:** Use `sqlx` for compile-time verified database queries.
- **Error Handling:** Use `thiserror` for domain and repository errors, wrapping infrastructure errors contextually.