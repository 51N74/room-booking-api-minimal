# Room Booking API (Minimal)

## 🚀 Project Description

[อ่านเป็นภาษาไทย](README.th.md)

Room Booking API (Minimal) is a Backend Service developed with **Rust** and the **Axum framework** for managing room bookings, users, and administrators. This API is designed to be highly efficient, secure, and **highly extensible** for various room booking applications.

## ✨ Features

* **User Management:**
    * Register new users (`POST /register`)
    * User login (`POST /login/user`)
* **Admin Management:**
    * Register new administrators (`POST /admin`)
    * Admin login (`POST /login/admin`)
    * View all users (`GET /admin/users`)
    * View user details by ID (`GET /admin/users/:user_id`)
    * Delete user (`DELETE /admin/users/:user_id`)
* **Room Management:**
    * Add rooms (`POST /admin/rooms`) - Admin only
    * View all active rooms (`GET /rooms/active`) - Public
    * View all rooms (`GET /rooms`) - Public
    * View room details by ID (`GET /rooms/:room_id`) - Public
    * Update room information (`PATCH /admin/rooms/:room_id`) - Admin only
    * Delete room (`DELETE /admin/rooms/:room_id`) - Admin only
* **Booking Management:**
    * Create a room booking (`POST /bookings`) - Requires Login (User)
    * Cancel a room booking (`DELETE /bookings/:id`) - Requires Login (User)
    * View all bookings for the logged-in user (`GET /bookings/user`) - Requires Login (User)
    * View all bookings in the system (`GET /admin/bookings`) - Admin only
* **Authentication & Authorization:**
    * Uses JWT (JSON Web Tokens) for authentication.
    * Role-Based Access Control for User and Admin roles.
* **State Management:**
    * Uses `axum::Extension` for managing `AppState` across the application.

## 🛠️ Technologies Used

* **Backend:** [Rust](https://www.rust-lang.org/)
* **Web Framework:** [Axum](https://docs.rs/axum/latest/axum/)
* **Database:** [SQLite](https://www.sqlite.org/index.html)
* **ORM/Database Toolkit:** [Diesel](https://diesel.rs/)
* **Asynchronous Runtime:** [Tokio](https://tokio.rs/)
* **JWT Handling:** [jsonwebtoken](https://crates.io/crates/jsonwebtoken)
* **Environment Variables:** [dotenv](https://crates.io/crates/dotenv)

## 📂 Project Structure

This project is organized following Domain-Driven Design (DDD) and Clean Architecture principles for flexibility and maintainability:

```

room_booking_api_minimal/
├── src/
│   ├── application/     # Business logic and Service layer (ex: user_service.rs, room_service.rs)
│   ├── domain/          # Domain models, Entities, Value Objects (ex: user.rs, room.rs, booking.rs)
│   ├── infrastructure/  # External connections (Database, JWT implementation, Repositories)
│   │   ├── admin_repository.rs
│   │   ├── database.rs
│   │   ├── jwt.rs
│   │   ├── room_repository.rs
│   │   └── user_repository.rs
│   ├── middleware/      # Middleware for Authentication, Authorization
│   │   └── auth.rs
│   ├── presentation/    # HTTP Handlers and AppState
│   │   ├── admin_handler.rs
│   │   ├── admin_user_handler.rs
│   │   ├── app_state.rs
│   │   ├── booking_handler.rs
│   │   ├── room_handler.rs
│   │   ├── test_handler.rs
│   │   └── user_handler.rs
│   └── main.rs          # Application entry point, Router Configuration
├── .env.example         # Example Environment Variables file
├── Cargo.toml           # Rust Dependencies Configuration
├── Diesel.toml          # Diesel CLI Configuration
└── README.md            # This documentation file
```

## ⚙️ Setup and Installation

### 1. Install Rust

If you don't have Rust installed on your system, please follow the instructions from the official website:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
#After installation, close and reopen your Terminal.
```

### 2. Install Diesel CLI

For managing Database Migrations with Diesel, you need to install `diesel_cli`:

```
cargo install diesel_cli --no-default-features --features "sqlite"
# If you are using other databases (e.g., postgres), change features to "postgres" or "mysql"
```

### 3. Set Up Environment Variables

Create a `.env` file in the project's Root Directory (same location as `Cargo.toml`) by copying the content from `.env.example` and replacing the values with your information:

Code snippet

```
DATABASE_URL="sqlite://database.db" # Example for SQLite
JWT_SECRET="[Specify a complex Secret Key for your JWT Token]"
```

**Example `JWT_SECRET`:** You can generate one using online tools (e.g., `uuidgen` or `openssl rand -base64 32` in Terminal) or use a sufficiently complex and long string.

### 4. Run Database Migrations

Once `DATABASE_URL` is configured in `.env`, run migrations to create tables in the Database:


```
diesel setup
diesel migration run
```

### 5. Run the Project

After all settings are complete, you can run the application using the command:

```
cargo run
```

The server will start at http://0.0.0.0:3000.

## 🧪 (Testing the API)
This project is **ready for basic use.** You can test the API using tools such as [Postman](https://www.postman.com/), [Insomnia](https://insomnia.rest/), หรือ [Thunder Client](https://marketplace.visualstudio.com/items?itemName=rangav.thunder-client) (VS Code Extension).

This project does not provide initial data (Seed Data). You can create users, administrators, rooms, and bookings yourself via the provided Endpoints.

### Key Endpoints Examples:

* **Public Access:**
    * `POST /register`
    * `POST /login/user`
    * `POST /admin`
    * `POST /login/admin`
    * `GET /rooms/active`
    * `GET /rooms`
    * `GET /rooms/:room_id`
* **Protected (Admin Access - Use Admin's JWT Token):**
    * `POST /admin/rooms`
    * `PATCH /admin/rooms/:room_id`
    * `DELETE /admin/rooms/:room_id`
    * `GET /admin/bookings`
    * `GET /admin/users`
    * `GET /admin/users/:user_id`
    * `DELETE /admin/users/:user_id`
    * `GET /admin/test-admin`
* **Protected (User Access - Use User's JWT Token):**
    * `POST /bookings`
    * `DELETE /bookings/:id`
    * `GET /bookings/user`
    * `GET /bookings/test-user`

## Roadmap

- Logout system
- More sophisticated booking status management
- Future integration with Frontend Application
- [Consider adding: More complex room search/filtering criteria]
- [Consider adding: Real-time room availability checking]

## ✉️ Contact

If you have any questions or need further assistance, please feel free to contact:

- **Email:** `jptns@proton.me`
- **Nostr:** `npub1esyenpe4c8ndsex8wdepnd964eytwkrr98pdk6qvvha3kk96pwyspyyes3`

## 📜 License

This project is licensed under the [MIT License](https://opensource.org/licenses/MIT). See the `LICENSE` file in the Root Directory for more details.
