# Room Booking API (Minimal)

## 🚀 คำอธิบายโปรเจกต์

Room Booking API (Minimal) เป็น Backend Service ที่พัฒนาด้วย **Rust** และ **Axum framework** สำหรับจัดการระบบจองห้องพัก, ผู้ใช้, และผู้ดูแลระบบ. API นี้ถูกออกแบบมาให้มีประสิทธิภาพสูง, ปลอดภัย, และสามารถ **นำไปต่อยอดได้หลากหลาย** สำหรับแอปพลิเคชันจองห้องพักต่างๆ.

## ✨ คุณสมบัติหลัก (Features)

* **User Management:**
    * ลงทะเบียนผู้ใช้ใหม่ (`POST /register`)
    * เข้าสู่ระบบผู้ใช้ (`POST /login/user`)
* **Admin Management:**
    * ลงทะเบียนผู้ดูแลระบบ (`POST /admin`)
    * เข้าสู่ระบบผู้ดูแลระบบ (`POST /login/admin`)
    * ดูรายการผู้ใช้ทั้งหมด (`GET /admin/users`)
    * ดูข้อมูลผู้ใช้ตาม ID (`GET /admin/users/:user_id`)
    * ลบผู้ใช้ (`DELETE /admin/users/:user_id`)
* **Room Management:**
    * เพิ่มห้องพัก (`POST /admin/rooms`) - เฉพาะ Admin
    * ดูห้องพักที่ใช้งานอยู่ทั้งหมด (`GET /rooms/active`) - Public
    * ดูห้องพักทั้งหมด (`GET /rooms`) - Public
    * ดูข้อมูลห้องพักตาม ID (`GET /rooms/:room_id`) - Public
    * อัปเดตข้อมูลห้องพัก (`PATCH /admin/rooms/:room_id`) - เฉพาะ Admin
    * ลบห้องพัก (`DELETE /admin/rooms/:room_id`) - เฉพาะ Admin
* **Booking Management:**
    * สร้างการจองห้องพัก (`POST /bookings`) - ต้อง Login (User)
    * ยกเลิกการจองห้องพัก (`DELETE /bookings/:id`) - ต้อง Login (User)
    * ดูรายการการจองทั้งหมดของผู้ใช้ที่ Login (`GET /bookings/user`) - ต้อง Login (User)
    * ดูรายการการจองทั้งหมดในระบบ (`GET /admin/bookings`) - เฉพาะ Admin
* **Authentication & Authorization:**
    * ใช้ JWT (JSON Web Tokens) สำหรับการยืนยันตัวตน.
    * การควบคุมการเข้าถึงตามบทบาท (Role-Based Access Control) สำหรับ User และ Admin.
* **State Management:**
    * ใช้ `axum::Extension` ในการจัดการ `AppState` ทั่วทั้งแอปพลิเคชัน.

## 🛠️ เทคโนโลยีที่ใช้ (Technologies Used)

* **Backend:** [Rust](https://www.rust-lang.org/)
* **Web Framework:** [Axum](https://docs.rs/axum/latest/axum/)
* **Database:** [SQLite](https://www.sqlite.org/index.html)
* **ORM/Database Toolkit:** [Diesel](https://diesel.rs/)
* **Asynchronous Runtime:** [Tokio](https://tokio.rs/)
* **JWT Handling:** [jsonwebtoken](https://crates.io/crates/jsonwebtoken)
* **Environment Variables:** [dotenv](https://crates.io/crates/dotenv)

## 📂 โครงสร้างโปรเจกต์ (Project Structure)


โปรเจกต์นี้ถูกจัดระเบียบตามแนวคิด Domain-Driven Design (DDD) และ Clean Architecture เพื่อความยืดหยุ่นและบำรุงรักษาได้ง่าย:
```

room_booking_api_minimal/
├── src/
│   ├── application/     # Business logic และ Service layer (ex: user_service.rs, room_service.rs)
│   ├── domain/          # โดเมนโมเดล, Entities, Value Objects (ex: user.rs, room.rs, booking.rs)
│   ├── infrastructure/  # การเชื่อมต่อภายนอก (Database, JWT implementation, Repositories)
│   │   ├── admin_repository.rs
│   │   ├── database.rs
│   │   ├── jwt.rs
│   │   ├── room_repository.rs
│   │   └── user_repository.rs
│   ├── middleware/      # Middleware สำหรับ Authentication, Authorization
│   │   └── auth.rs
│   ├── presentation/    # HTTP Handlers และ AppState
│   │   ├── admin_handler.rs
│   │   ├── admin_user_handler.rs
│   │   ├── app_state.rs
│   │   ├── booking_handler.rs
│   │   ├── room_handler.rs
│   │   ├── test_handler.rs
│   │   └── user_handler.rs
│   └── main.rs          # จุดเริ่มต้นของแอปพลิเคชัน, Router Configuration
├── .env.example         # ตัวอย่างไฟล์ Environment Variables
├── Cargo.toml           # การตั้งค่า Dependencies ของ Rust
├── Diesel.toml          # การตั้งค่า Diesel CLI
└── README.md            # ไฟล์เอกสารนี้
```

## ⚙️ การตั้งค่าและติดตั้ง (Setup and Installation)

### 1. ติดตั้ง Rust

หากคุณยังไม่มี Rust ติดตั้งอยู่ในระบบ โปรดทำตามคำแนะนำจากเว็บไซต์ทางการ:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
หลังจากติดตั้งเสร็จ ให้ปิด Terminal แล้วเปิดใหม่.
```

### 2. ติดตั้ง Diesel CLI

สำหรับการจัดการ Database Migrations ด้วย Diesel คุณต้องติดตั้ง `diesel_cli`:

```
cargo install diesel_cli --no-default-features --features "sqlite"
# หากคุณใช้ database อื่นๆ (เช่น postgres) ให้เปลี่ยน features เป็น "postgres" หรือ "mysql"
```

### 3. ตั้งค่า Environment Variables

สร้างไฟล์ `.env` ใน Root Directory ของโปรเจกต์ (ที่เดียวกับ `Cargo.toml`) โดยคัดลอกเนื้อหาจาก `.env.example` และแทนที่ค่าด้วยข้อมูลของคุณ:

Code snippet

```
DATABASE_URL="sqlite://database.db" # ตัวอย่างสำหรับ SQLite
JWT_SECRET="[ระบุ Secret Key ที่ซับซ้อนสำหรับ JWT Token ของคุณ]"
```

**ตัวอย่าง `JWT_SECRET`:** คุณสามารถสร้างได้ด้วยเครื่องมือออนไลน์ (เช่น `uuidgen` หรือ `openssl rand -base64 32` ใน Terminal) หรือใช้สตริงที่ซับซ้อนและยาวพอสมควร.

### 4. รัน Database Migrations

เมื่อตั้งค่า `DATABASE_URL` ใน `.env` เรียบร้อยแล้ว ให้รัน Migrations เพื่อสร้างตารางใน Database:

```
diesel setup
diesel migration run
```

### 5. รันโปรเจกต์

หลังจากตั้งค่าทั้งหมดแล้ว คุณสามารถรันแอปพลิเคชันได้โดยใช้คำสั่ง:

```
cargo run
```

เซิร์ฟเวอร์จะเริ่มต้นที่ http://0.0.0.0:3000.

## 🧪 การทดสอบ API (Testing the API)

โปรเจกต์นี้ **พร้อมใช้งานเบื้องต้น** คุณสามารถทดสอบ API โดยใช้เครื่องมือเช่น [Postman](https://www.postman.com/), [Insomnia](https://insomnia.rest/), หรือ [Thunder Client](https://marketplace.visualstudio.com/items?itemName=rangav.thunder-client) (VS Code Extension).

โปรเจกต์นี้ไม่มีข้อมูลเริ่มต้น (Seed Data) ให้ คุณสามารถสร้างข้อมูลผู้ใช้, ผู้ดูแลระบบ, ห้องพัก, และการจองได้เองผ่าน Endpoints ที่มีให้.

### ตัวอย่าง Endpoints ที่สำคัญ:

* **Public Access:**
    * `POST /register`
    * `POST /login/user`
    * `POST /admin`
    * `POST /login/admin`
    * `GET /rooms/active`
    * `GET /rooms`
    * `GET /rooms/:room_id`
* **Protected (Admin Access - ใช้ JWT Token ของ Admin):**
    * `POST /admin/rooms`
    * `PATCH /admin/rooms/:room_id`
    * `DELETE /admin/rooms/:room_id`
    * `GET /admin/bookings`
    * `GET /admin/users`
    * `GET /admin/users/:user_id`
    * `DELETE /admin/users/:user_id`
    * `GET /admin/test-admin`
* **Protected (User Access - ใช้ JWT Token ของ User):**
    * `POST /bookings`
    * `DELETE /bookings/:id`
    * `GET /bookings/user`
    * `GET /bookings/test-user`

## แผนการในอนาคต (Roadmap)

- ระบบ Logout
- การจัดการสถานะการจอง (Booking Status) ที่ซับซ้อนยิ่งขึ้น
- การเชื่อมต่อกับ Frontend Application ในอนาคต
- [พิจารณาเพิ่ม: การค้นหา/กรองห้องพักตามเงื่อนไขที่ซับซ้อนขึ้น]
- [พิจารณาเพิ่ม: การตรวจสอบห้องว่างแบบเรียลไทม์]

## ✉️ ติดต่อ (Contact)

หากมีข้อสงสัยหรือต้องการความช่วยเหลือเพิ่มเติม สามารถติดต่อได้ที่:

- **Email:** `jptns@proton.me`
- **Nostr:** `npub1esyenpe4c8ndsex8wdepnd964eytwkrr98pdk6qvvha3kk96pwyspyyes3`

## 📜 License

โปรเจกต์นี้อยู่ภายใต้ [MIT License](https://opensource.org/licenses/MIT). ดูรายละเอียดเพิ่มเติมได้ที่ไฟล์ `LICENSE` ใน Root Directory.
