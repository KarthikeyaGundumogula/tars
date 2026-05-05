# FrameHouse Backend

A high-performance Rust backend powering **FrameHouse**, a cinematic platform where fans share edits, posters, and theories about their favorite movies and series. Built with deterministic paging for precise frontend layout engines and optimized for performance and spatial data organization.

## Table of Contents

- [Overview](#overview)
- [Tech Stack](#tech-stack)
- [Project Structure](#project-structure)
- [Database Schema](#database-schema)
- [API Routes](#api-routes)
- [Test Infrastructure](#test-infrastructure)
- [Configuration](#configuration)
- [Getting Started](#getting-started)

---

## Overview

### What is FrameHouse?

FrameHouse is a creative platform where:

- **Users** (Profiles) register as Artists/Makers and build "Presence" by creating works
- **Works** are creative submissions categorized as:
  - **Edits**: Cinematic video edits with platform sources (YouTube, Twitter, Native)
  - **Posters**: Visual artwork with different format specifications (Canvas, Standard, Square, Vertical)
  - **Scripts**: Long-form theories/scripts with image context (up to 10 images)
- **Originals**: Movies or Series that act as focal anchors for all works (e.g., _RRR_, _OG_)
- **Roles**: Artists/Makers associated with originals in specific capacities (director, music composer, actor, etc.)

### Key Features

- **Deterministic Paging**: Precision payload structures for virtualized CSS Grid layouts on frontend
- **Role-Based Relationships**: Track multiple roles per artist per original (STAR = actor, MAKER = creator)
- **Presence System**: Reputation/credit system that rewards creators
- **Multi-Profile Support**: YouTube, Twitter, Instagram social profile linking
- **Admin Layer**: Admin authentication with password hashing
- **Watchlist Tracking**: Personal tracking of originals with status (WATCHED, WATCHING, WANT_TO_WATCH)

---

## Tech Stack

| Component            | Technology                               |
| -------------------- | ---------------------------------------- |
| **Runtime**          | Rust 2024 Edition                        |
| **Web Framework**    | Axum 0.8.8                               |
| **Database**         | PostgreSQL with SQLx 0.8.6               |
| **Authentication**   | JWT (jsonwebtoken 10.3.0)                |
| **Password Hashing** | Argon2 with cryptographic salting        |
| **Async Runtime**    | Tokio (full features)                    |
| **Serialization**    | Serde/serde_json                         |
| **Testing**          | Tokio test, reqwest client, sqlx queries |
| **Configuration**    | config crate + YAML files                |

---

## Project Structure

```
tars/
├── Cargo.toml              # Project manifest with dependencies
├── configuration.yaml      # App configuration (port, DB settings)
├── Prd.md                  # Product requirements and API specs
├── README.md               # This file
├── rules.md                # Business rules and constraints
├── migrations/             # SQL migrations (sqlx managed)
│   ├── 20260420124048_create_tables.sql       # Core tables & enums
│   ├── 20260424083449_admin_and_altering.sql  # Admin & watchlist system
│   ├── 20260428141101_profile_and_orignals_alter.sql  # Profile enhancements
│   ├── 20260428154016_not_null_on_profile_type.sql   # Schema refinements
│   └── 20260428194054_orignals_metadata_columns.sql  # Additional metadata
├── scripts/
│   └── init_db.sh          # Database initialization script
├── src/
│   ├── main.rs             # Application entry point
│   ├── lib.rs              # Library root with module exports
│   ├── startup.rs          # Server startup & route configuration
│   ├── configuration.rs    # Config loading and database settings
│   ├── db/                 # Database access layer
│   │   ├── mod.rs
│   │   ├── artists.rs      # Artist/Profile queries
│   │   ├── works.rs        # Works queries
│   │   └── originals.rs    # Originals queries
│   ├── errors/             # Error handling & API error types
│   │   └── mod.rs
│   ├── routes/             # HTTP endpoint handlers
│   │   ├── mod.rs
│   │   ├── artists.rs      # POST /artist/register
│   │   ├── health_check.rs # GET /health_check
│   │   ├── works.rs        # Work submission endpoints
│   │   ├── originals.rs    # Original management endpoints
│   │   └── originals.rs    # Artist profile endpoints
│   ├── types/              # Data models and schemas
│   │   ├── mod.rs
│   │   ├── db/             # Database entity types
│   │   │   ├── mod.rs
│   │   │   ├── profile.rs  # Profile/User model
│   │   │   ├── work.rs     # Work entity types
│   │   │   └── original.rs # Original entity type
│   │   ├── requests/       # HTTP request schemas
│   │   │   ├── mod.rs
│   │   │   ├── auth.rs     # Registration & login payloads
│   │   │   └── works.rs    # Work creation payloads
│   │   └── response/       # HTTP response schemas
│   │       └── mod.rs
│   └── utils/              # Utility functions
│       ├── mod.rs
│       └── password.rs     # Argon2 hashing & verification
├── tests/                  # Integration tests
│   ├── auth.rs             # Authentication tests
│   ├── health_check.rs     # Health endpoint test
│   ├── upload_works.rs     # Work submission tests
│   └── utils/
│       ├── mod.rs
│       ├── spawn_app.rs    # Test server spawning utility
│       └── postgres_config.rs  # Test database configuration
└── target/                 # Build artifacts (excluded from repo)
```

---

## Database Schema

### Enums

#### `role_type`

```
STAR, MAKER
```

- **STAR**: Artist role (actors, performers)
- **MAKER**: Creator role (directors, composers, writers, etc.)

#### `supported_platforms`

```
YOUTUBE, TWITTER, NATIVE
```

External platforms where video edits are hosted.

#### `edit_format`

```
IMAX-VIDEO, ACADEMY-VIDEO, SQUARE-VIDEO, VERTICAL-VIDEO
```

Video aspect ratios for deterministic layout rendering.

#### `poster_format`

```
CANVAS-POSTER, STANDARD-POSTER, SQUARE-POSTER, VERTICAL-POSTER
```

Poster dimensions for consistent grid layouts.

#### `work_category`

```
EDIT, POSTER, SCRIPT
```

Types of creative works users can submit.

#### `profile_type`

```
STAR, MAKER, ARTIST
```

Profile classification for permission and feature access.

#### `watchlist_status`

```
WATCHED, WATCHING, WANT_TO_WATCH
```

User's tracking status for an original.

---

### Tables

#### `originals`

**Purpose**: Anchor movies/series that works reference

| Column            | Type        | Constraints   | Notes                            |
| ----------------- | ----------- | ------------- | -------------------------------- |
| `id`              | UUID        | PK            | Unique identifier                |
| `title`           | VARCHAR     | NOT NULL      | Movie/Series name                |
| `description`     | TEXT        | NOT NULL      | Plot synopsis                    |
| `cover_img`       | VARCHAR     | NOT NULL      | CDN cover image URL              |
| `presence`        | BIGINT      | DEFAULT 100   | Cumulative community engagement  |
| `password_hash`   | TEXT        | NOT NULL      | Admin password for this original |
| `created_at`      | TIMESTAMPTZ | DEFAULT NOW() | Creation timestamp               |
| `associated_with` | UUID        | FK→profiles   | Producer/studio profile          |

---

#### `profiles`

**Purpose**: User accounts (Artists/Makers/Fans)

| Column              | Type         | Constraints      | Notes                     |
| ------------------- | ------------ | ---------------- | ------------------------- |
| `id`                | UUID         | PK               | Unique user ID            |
| `user_name`         | VARCHAR(50)  | UNIQUE, NOT NULL | Display name              |
| `tag_line`          | TEXT         | NOT NULL         | Bio/motto                 |
| `is_claimed`        | BOOLEAN      | NOT NULL         | Admin verification status |
| `profile_picture`   | TEXT         | NOT NULL         | Avatar image URL          |
| `password_hash`     | TEXT         | NOT NULL         | Argon2 hashed password    |
| `presence`          | BIGINT       | DEFAULT 100      | Reputation score          |
| `profile_type`      | profile_type | DEFAULT 'ARTIST' | STAR/MAKER/ARTIST         |
| `youtube_profile`   | TEXT         |                  | YouTube channel URL       |
| `twitter_profile`   | TEXT         |                  | Twitter handle URL        |
| `instagram_profile` | TEXT         |                  | Instagram profile URL     |
| `created_at`        | TIMESTAMPTZ  | DEFAULT NOW()    | Registration date         |

**Constraints**:

- Unique constraint on `user_name` per migration requirement

---

#### `roles`

**Purpose**: Association of artists to originals in specific capacities

| Column        | Type        | Constraints      | Notes                                          |
| ------------- | ----------- | ---------------- | ---------------------------------------------- |
| `profile_id`  | UUID        | FK→profiles, PK  | Artist reference                               |
| `original_id` | UUID        | FK→originals, PK | Movie/series reference                         |
| `category`    | role_type   | PK               | STAR or MAKER                                  |
| `role_name`   | TEXT        | PK               | Specific role (e.g., "Director", "Lead Actor") |
| `created_at`  | TIMESTAMPTZ | DEFAULT NOW()    | Role assignment date                           |

**Constraints**:

- Composite PK: `(profile_id, original_id, role_name)`
- ON DELETE CASCADE for both FKs

---

#### `works`

**Purpose**: Creative submissions (edits, posters, scripts)

| Column       | Type          | Constraints           | Notes                |
| ------------ | ------------- | --------------------- | -------------------- |
| `id`         | UUID          | PK                    | Work identifier      |
| `title`      | VARCHAR       |                       | Display title        |
| `artist_id`  | UUID          | FK→profiles, NOT NULL | Creator profile      |
| `category`   | work_category | NOT NULL              | EDIT/POSTER/SCRIPT   |
| `credits`    | BIGINT        | DEFAULT 0             | Reward points earned |
| `created_at` | TIMESTAMPTZ   | DEFAULT NOW()         | Submission date      |

---

#### `originals_credits`

**Purpose**: Many-to-many link between works and originals

| Column        | Type | Constraints      | Notes              |
| ------------- | ---- | ---------------- | ------------------ |
| `work_id`     | UUID | FK→works, PK     | Work reference     |
| `original_id` | UUID | FK→originals, PK | Original reference |

**Constraints**:

- Composite PK: `(work_id, original_id)`
- Enables one work to reference multiple originals

---

#### `edits`

**Purpose**: Video edit metadata (extends works table)

| Column       | Type                | Constraints             | Notes                                           |
| ------------ | ------------------- | ----------------------- | ----------------------------------------------- |
| `work_id`    | UUID                | FK→works, PK            | Edit identifier                                 |
| `src_id`     | VARCHAR             | NOT NULL                | Platform-specific video ID (YouTube UUID, etc.) |
| `platform`   | supported_platforms | NOT NULL                | YOUTUBE/TWITTER/NATIVE                          |
| `format`     | edit_format         | DEFAULT 'ACADEMY-VIDEO' | Aspect ratio for layout                         |
| `created_at` | TIMESTAMPTZ         | DEFAULT NOW()           | Creation date                                   |

---

#### `posters`

**Purpose**: Image poster metadata (extends works table)

| Column       | Type          | Constraints               | Notes                        |
| ------------ | ------------- | ------------------------- | ---------------------------- |
| `work_id`    | UUID          | FK→works, PK              | Poster identifier            |
| `src_id`     | VARCHAR       | NOT NULL                  | CDN/storage image identifier |
| `format`     | poster_format | DEFAULT 'STANDARD-POSTER' | Dimensions for grid          |
| `created_at` | TIMESTAMPTZ   | DEFAULT NOW()             | Creation date                |

---

#### `scripts`

**Purpose**: Long-form theory/script metadata (extends works table)

| Column        | Type        | Constraints               | Notes                             |
| ------------- | ----------- | ------------------------- | --------------------------------- |
| `work_id`     | UUID        | FK→works, PK              | Script identifier                 |
| `img_src_ids` | VARCHAR[]   | CHECK (array_length ≤ 10) | Support images (max 10)           |
| `thoughts`    | TEXT[]      |                           | Array of theory/script paragraphs |
| `created_at`  | TIMESTAMPTZ | DEFAULT NOW()             | Creation date                     |

---

#### `admins`

**Purpose**: Administrative user accounts

| Column                | Type        | Constraints   | Notes                  |
| --------------------- | ----------- | ------------- | ---------------------- |
| `admin_id`            | UUID        | PK            | Admin ID               |
| `admin_name`          | TEXT        | NOT NULL      | Display name           |
| `admin_password_hash` | TEXT        | NOT NULL      | Argon2 hashed password |
| `created_at`          | TIMESTAMPTZ | DEFAULT NOW() | Creation date          |

---

#### `beta_whitelist`

**Purpose**: Early access control for beta features

| Column            | Type        | Constraints   | Notes                           |
| ----------------- | ----------- | ------------- | ------------------------------- |
| `artist_username` | TEXT        |               | Whitelisted username            |
| `is_claimed`      | BOOLEAN     | NOT NULL      | Whether beta access was claimed |
| `added_at`        | TIMESTAMPTZ | DEFAULT NOW() | Whitelist date                  |

---

#### `ledger`

**Purpose**: Personal watchlist and engagement tracking per user/original

| Column            | Type             | Constraints             | Notes                               |
| ----------------- | ---------------- | ----------------------- | ----------------------------------- |
| `original_id`     | UUID             | FK→originals, PK        | Movie/series reference              |
| `profile_id`      | UUID             | FK→profiles, PK         | User reference                      |
| `pub_visibility`  | BOOLEAN          | DEFAULT true            | Public vs private tracking          |
| `tagged_works`    | UUID[]           |                         | Works user tagged for this original |
| `pre_thought`     | TEXT             |                         | Initial impression before watching  |
| `post_impression` | TEXT             |                         | Thoughts after watching             |
| `status`          | watchlist_status | DEFAULT 'WANT_TO_WATCH' | WATCHED/WATCHING/WANT_TO_WATCH      |

**Constraints**:

- Composite PK: `(profile_id, original_id)`

---

## API Routes

### Implemented Endpoints

#### `POST /artist/register`

Register a new artist/user profile

**Request Body** (ProfileSignup):

```json
{
  "user_name": "kapten",
  "tag_line": "I will never care for you",
  "password": "kapten@1023",
  "profile_picture": "https://cdn.example.com/pic.jpg",
  "youtube_profile": "https://youtube.com/c/kapten",
  "twitter_profile": "@kapten",
  "instagram_profile": "kapten_official"
}
```

**Response**:

- `200 OK` on success
- `422 UNPROCESSABLE_ENTITY` on invalid data
- `500 INTERNAL_SERVER_ERROR` on database errors

**Features**:

- Password hashed with Argon2 (cryptographic salt generation)
- New UUID generated for profile
- Default presence set to 100
- Profile type defaults to ARTIST
- Social profiles are optional

---

#### `GET /health_check`

System health status

**Response**:

- `200 OK` with empty body

**Purpose**: Load balancer probe and deployment verification

---

#### `POST /works/new/{work_type}`

Submit a new creative work

**Route Parameters**:

- `work_type`: EDIT, POSTER, or SCRIPT

**Expected Functionality** (handler exists but impl in progress):

- Validate work submission
- Create work entry
- Type-specific metadata handling (video src, image format, script content)
- Associate with originals via `originals_credits`
- Award credits to artist

---

#### `POST /originals/new` (Tested but route not fully implemented)

Register a new original (movie/series)

**Request Body** (from tests):

```json
{
  "title": "They Call him Og",
  "password": "1234",
  "associated_with": "DVV Entertainments (UUID)",
  "actors": [
    {
      "role": "Character Name",
      "artist": "Actor Name"
    }
  ],
  "makers": [
    {
      "role": "Director",
      "artist": "Director Name"
    }
  ]
}
```

---

### Planned Endpoints (from PRD)

#### `GET /api/works`

Fetch global works feed with deterministic paging

**Query Parameters**:

```
?page=1&limit=5&category=Edit&clientPlatform=mobile
```

**Response Structure**:

```json
{
  "success": true,
  "data": [
    {
      "id": "w-uuid-xyz",
      "title": "OG Intro Blast",
      "category": "Edit",
      "type": "video",
      "aspectRatio": 1.77,
      "credits": 2480,
      "artist": "PowerStar_FC",
      "artistId": "art-uuid-1",
      "originalId": "og-uuid",
      "image": "https://cloudfront.net/posters/poster1.jpg",
      "platform": "youtube",
      "platformId": "uuid"
    }
  ],
  "meta": {
    "totalCount": 12040,
    "nextCursor": "uuid-cursor-hash"
  }
}
```

#### `GET /api/originals`

Fetch anchor originals (movies/series)

**Response Structure**:

```json
{
  "success": true,
  "data": [
    {
      "id": "og-original",
      "title": "OG",
      "description": "...",
      "coverImage": "https://cdn.framehouse.com/covers/og.png",
      "stats": {
        "presence": 2480,
        "members": 1620,
        "releases": 18
      }
    }
  ]
}
```

#### `GET /api/artists/:artistId`

Fetch artist profile with work history

---

## Test Infrastructure

### Architecture

Tests are integration tests that:

1. Spawn isolated server instances for each test
2. Create separate PostgreSQL databases per test run
3. Verify HTTP endpoints + database state
4. Clean up after execution (cascade deletes via FK constraints)

### Test Utilities

#### `spawn_app.rs`

```rust
pub async fn spawn() -> TestApp {
    // 1. Bind to random port (0 = OS-assigned)
    // 2. Create unique DB name via UUID
    // 3. Run migrations on isolated DB
    // 4. Spawn server task
    // Returns: TestApp { address, db_pool }
}
```

**Key Features**:

- Random port assignment enables parallel test execution
- Unique UUID database names prevent test isolation conflicts
- Database configuration uses environment DATABASE_URL

#### `postgres_config.rs`

Database connection pooling and migration setup for tests

---

### Existing Tests

#### `tests/health_check.rs`

```rust
#[tokio::test]
async fn health_check_test()
```

**Verifies**:

- Server starts on random port
- GET /health_check returns 200 OK
- Response body is empty (content-length = 0)

**Pattern**:

```rust
let app = spawn_app::spawn().await;
let client = Client::new();
let response = client.get(&format!("{}/health_check", app.address)).send().await;
assert!(response.status().is_success());
```

---

#### `tests/auth.rs`

Comprehensive authentication/registration tests

##### `register_profile_return_200_on_correct_data`

```rust
POST /artist/register {
  "user_name": "kapten",
  "tag_line": "...",
  "password": "...",
  "profile_picture": "...",
  "youtube_profile": "..."
}
```

**Assertions**:

- HTTP response is 2xx success
- Database contains inserted profile with matching youtube_profile
- Demonstrates sqlx macro queries: `sqlx::query_scalar!`

---

##### `register_profile_return_error_on_incorrect_data`

```rust
POST /artist/register {
  "user_name": "kapten",
  "password": "kapten@1023"
  // Missing required: tag_line, profile_picture
}
```

**Assertions**:

- HTTP response is 4xx client error
- JSON deserialization fails as expected

---

##### `register_original_return_success_on_correct_data`

```rust
POST /originals/new {
  "title": "They Call him Og",
  "password": "1234",
  "actors": [{ "role": "...", "artist": "..." }],
  "makers": [{ "role": "...", "artist": "..." }]
}
```

**Status**: Test defined but route implementation not visible in codebase yet

---

#### `tests/upload_works.rs`

Work submission tests (file exists but details not examined)

---

### Running Tests

```bash
# Run all tests
cargo test --test '*'

# Run specific test
cargo test --test auth register_profile_return_200_on_correct_data

# Run with output
cargo test -- --nocapture

# Run sequentially (avoid port conflicts if needed)
cargo test -- --test-threads=1
```

**Environment Requirements**:

```bash
export DATABASE_URL="postgres://postgres:Kap10@localhost:5432/test_db"
```

---

## Error Handling

### ApiError Enum

```rust
pub enum ApiError {
    NotFound,                    // 500 Internal Server Error
    Serailization(JsonRejection),// 422 Unprocessable Entity (JSON parse fails)
    DbError(sqlx::Error),        // 500 Internal Server Error (DB operations)
    Argon2Error(...),            // 500 Internal Server Error (password hash fails)
}
```

**Response Format**:

```json
{
  "status_code": "422",
  "message": "Uploads are missing some field"
}
```

---

## Configuration

### YAML Configuration (`configuration.yaml`)

```yaml
application_port: 8080
database:
  host: localhost
  port: 5432
  username: "postgres"
  password: "Kap10"
  database_name: "aera"
```

### Environment Variables

```bash
DATABASE_URL=postgres://username:password@host:port/database_name
```

**Priority**:

1. Environment variable DATABASE_URL (used if present)
2. Configuration file settings fallback
3. Defaults in configuration.rs

### Configuration Code

```rust
pub struct Settings {
    pub application_port: u16,
    pub database: DatabaseSettings,
}

pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}
```

---

## Getting Started

### Prerequisites

- Rust 1.70+ (uses 2024 edition)
- PostgreSQL 12+
- Tokio async runtime support

### Installation

1. **Clone repository**:

   ```bash
   git clone <repo-url>
   cd tars
   ```

2. **Install dependencies**:

   ```bash
   cargo build
   ```

3. **Setup PostgreSQL**:

   ```bash
   # Create main database
   createdb aera

   # Run migrations
   sqlx database create
   sqlx migrate run
   ```

4. **Configure environment**:

   ```bash
   # Create .env file or export
   export DATABASE_URL="postgres://postgres:Kap10@localhost:5432/aera"
   ```

5. **Run server**:
   ```bash
   cargo run
   # Server starts at http://127.0.0.1:8080
   ```

### Testing

```bash
# Run full test suite
cargo test

# Run specific test category
cargo test --test auth
cargo test --test health_check

# With logging
RUST_LOG=debug cargo test -- --nocapture
```

### Development Workflow

1. **Run in development mode**:

   ```bash
   cargo watch -x run
   ```

2. **Check code style**:

   ```bash
   cargo fmt
   cargo clippy
   ```

3. **Add migrations**:
   ```bash
   sqlx migrate add -r <migration_name>
   # Edit file at migrations/TIMESTAMP_<name>.sql
   sqlx migrate run
   ```

---

## Architecture Decisions

### Why Axum?

- Lightweight, composable web framework
- Strong TypeScript type safety via Rust's type system
- Excellent error handling with extractors
- State management via Arc<T>

### Why SQLx?

- Compile-time SQL verification via macros
- Zero-cost abstractions
- Native PostgreSQL support with UUID/JSON features
- Seamless async/await integration

### Why Argon2?

- Memory-hard password hashing resists GPU attacks
- OWASP recommended algorithm
- Tokio-compatible async implementation

### Database Organization

- **Separate tables per work type** (EDITS, POSTERS, SCRIPTS) for type-specific constraints
- **Composite primary keys** on roles to prevent duplicate role assignments
- **Foreign key cascades** ensure referential integrity
- **UUID identifiers** enable distributed system scaling

---

## Security Considerations

✅ **Implemented**:

- Argon2 password hashing with cryptographic salts
- SQL injection prevention via parameterized queries (sqlx macros)
- Type-safe deserialization with serde

⚠️ **Planned/Future**:

- JWT token authentication (dependency added, not yet wired)
- CORS configuration for frontend cross-origin requests
- Rate limiting on endpoints
- Admin password-protected original mutations
- Profile verification flow

---

## Future Development

- [ ] Complete work submission endpoints (EDIT/POSTER/SCRIPT types)
- [ ] Implement global works feed with cursor-based pagination
- [ ] Artist profile endpoints with reputation system
- [ ] Watchlist management via ledger table
- [ ] JWT authentication middleware
- [ ] CDN integration for media serving
- [ ] Real-time presence updates
- [ ] Social profile verification

---

## License

[Specify your license]

## Contact

[Specify contact information]
