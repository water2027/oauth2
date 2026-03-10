# Project Overview: oauth2

A Rust-based OAuth2 and Identity service following Domain-Driven Design (DDD) principles. It manages user identities, authentication, and OAuth2 authorization flows using a clean architectural approach.

## Tech Stack
- **Language:** Rust (Edition 2024)
- **Runtime:** Tokio (Async/Await)
- **Database:** PostgreSQL (via SQLx)
- **Cache:** Redis
- **Serialization:** Serde / Serde JSON
- **Error Handling:** thiserror
- **Other:** Chrono (Time), async-trait

## Architecture & Structure
The project is organized into distinct contexts within the `src/` directory:

- **`src/context/`**: Contains the core domain logic.
  - **`identity/`**: Manages users, registration, login, sessions, and validation codes.
    - `application/`: Application services (orchestration).
    - `entity/`: Domain entities (User, Session, etc.).
    - `value_object/`: Immutable domain values (Email, UserID, etc.).
    - `service/`: Domain services.
    - `repository/`: Repository interfaces.
    - `infrastructure/`: Implementations of repositories (DB/Redis).
  - **`oauth/`**: Handles the OAuth2 flow (Authorization, Tokens).
- **`src/presentation/`**: The interface layer.
  - **`http/`**: HTTP API endpoints (likely using a web framework, though not explicitly in `Cargo.toml` yet, but structured for it).
- **`src/shared/`**: Common utilities, errors, and shared logic.

## Building and Running
- **Build:** `cargo build`
- **Run:** `cargo run`
- **Test:** `cargo test`
- **Database Migrations:** `sqlx migrate run` (Requires `sqlx-cli`)

## Development Conventions
- **DDD Compliance:** Ensure business logic stays within the `context/` domain layer. Use application services to orchestrate flows.
- **Error Handling:** Use `thiserror` for domain and repository errors. Domain errors should wrap repository errors where appropriate.
- **Type Safety:** Heavily utilize Value Objects (found in `src/context/*/value_object/`) to enforce business rules at the type level (e.g., `Email`, `HashedPassword`).
- **Async:** All I/O operations should be asynchronous using `tokio` and `sqlx`.
