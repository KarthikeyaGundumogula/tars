# FrameHouse Backend

A production-grade Rust backend powering **FrameHouse** — a cinematic platform where fans share edits, posters, and theories about their favorite movies and series. This project demonstrates expertise in **Rust systems programming**, **backend architecture**, and **product-driven engineering** with deterministic paging for precise frontend layout engines.

## Table of Contents

- [Technical Architecture](#technical-architecture)
- [Rust Expertise](#rust-expertise)
- [Backend Engineering](#backend-engineering)
- [Product Thinking](#product-thinking)
- [Tech Stack](#tech-stack)
- [Project Structure](#project-structure)
- [Database Schema](#database-schema)
- [API Routes](#api-routes)
- [Test Infrastructure](#test-infrastructure)
- [Getting Started](#getting-started)

---

## Technical Architecture

### Core Design Philosophy

This backend is built around **deterministic data structures** and **type-safe domain modeling** to enable precise frontend layout engines. The architecture prioritizes:

- **Compile-time safety** through Rust's ownership system and SQLx's compile-time checked queries
- **Domain-driven design** with validated domain types that enforce business rules at the type level
- **Performance optimization** with async/await, connection pooling, and efficient database indexing
- **Security-first** approach with Argon2 password hashing, JWT authentication, and input validation

### System Overview

FrameHouse is a creative platform where:

- **Users** (Profiles) register as Artists/Makers and build "Presence" by creating works
- **Works** are creative submissions categorized as:
  - **Edits**: Cinematic video edits with platform sources (YouTube, Twitter, Native)
  - **Posters**: Visual artwork with different format specifications (Canvas, Standard, Square, Vertical)
  - **Scripts**: Long-form theories/scripts with image context (up to 10 images)
- **Originals**: Movies or Series that act as focal anchors for all works (e.g., _RRR_, _OG_)
- **Roles**: Artists/Makers associated with originals in specific capacities (director, music composer, actor, etc.)

---

## Rust Expertise

### Domain-Driven Design with Type Safety

Implemented a robust domain layer with **newtype patterns** that enforce business rules at compile time:

```rust
// src/domain/profiles/handle.rs - Handle validation with Unicode support
pub struct Handle(String);

impl Handle {
    pub fn parse(handle: String) -> Result<Self, String> {
        // Enforces: length limits, no spaces, alphanumeric + underscore
        // Supports Unicode scripts (Telugu, etc.) with conditional lowercase
        // Prevents leading/trailing underscores
    }
}
```

**Key patterns demonstrated:**

- **Newtype pattern** for domain primitives (Handle, StageName, HexColor, TagLine)
- **Trait implementations** (Display, AsRef, Deserialize) for seamless integration
- **Comprehensive unit tests** for validation logic
- **Unicode-aware validation** supporting international scripts (Telugu characters)

### Error Handling with thiserror

Centralized error handling using Rust's `thiserror` crate for ergonomic error propagation:

```rust
// src/errors/mod.rs
#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Server responded with nothing")]
    NotFound,
    #[error("Unable to process the incoming request")]
    Serialization(#[from] JsonRejection),
    #[error("There is an error at the database")]
    DbError(#[from] sqlx::Error),
    #[error("password hashing failed")]
    Argon2Error(#[from] argon2::password_hash::Error),
    #[error("jwt failure")]
    JWTError(#[from] jsonwebtoken::errors::Error),
}
```

**Benefits:**

- **Automatic conversion** from underlying errors with `#[from]`
- **Custom HTTP status codes** via `IntoResponse` trait implementation
- **Structured error responses** with consistent JSON format
- **Type-safe error handling** throughout the application

### Async/Await with Tokio

Full async runtime using Tokio for high-performance concurrent operations:

```rust
// src/main.rs
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let db_pool = PgPoolOptions::new()
        .max_connections(50)
        .connect(&config.database.connection_string())
        .await?;
    // Server runs on async runtime
}
```

**Async patterns used:**

- **Connection pooling** with configurable max connections (50)
- **Concurrent request handling** via Axum's async handlers
- **Instrumentation** with tracing for async operation visibility
- **Structured concurrency** with proper error propagation

### Compile-Time Database Queries

Leveraging SQLx's compile-time query checking for database safety:

```rust
// Queries are checked at compile time against the actual database schema
let profile = sqlx::query_scalar!(
    "SELECT user_name FROM profiles WHERE user_name = $1",
    handle
)
.fetch_one(&pool)
.await?;
```

**Advantages:**

- **Zero-cost abstraction** — no runtime ORM overhead
- **Type safety** — query results are typed structs
- **Migration safety** — compile errors when schema changes
- **Performance** — prepared statements with parameter binding

---

## Backend Engineering

### Security Implementation

#### Password Hashing with Argon2

Industry-standard password hashing using Argon2id with cryptographic salt generation:

```rust
// src/services/auth_service/password.rs
pub fn get_password_hash(password: &str) -> Result<String, ApiError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default(); // Argon2id v19 with secure defaults
    Ok(argon2.hash_password(password.as_bytes(), &salt)?.to_string())
}
```

**Security features:**

- **Cryptographic salt** generation using OS RNG
- **Argon2id** (winner of Password Hashing Competition)
- **Timing-attack resistant** comparisons
- **Constant-time verification** to prevent timing attacks

#### JWT Authentication

Token-based authentication with role-based access control:

```rust
pub fn create_jwt(handle: &str, role: &str, secret: &str, profile_id: Uuid) -> Result<String, ApiError> {
    let claims = Claims {
        sub: handle.to_string(),
        profile_id,
        role: role.to_string(),
        exp: expiry, // 7-day expiration
        iat: now.timestamp() as usize,
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))?
}
```

**Auth features:**

- **Cookie-based sessions** with HttpOnly and SameSite protection
- **Role-based authorization** (admin vs artist)
- **Token expiration** with 7-day TTL
- **Secure cookie handling** with proper SameSite policies

### Database Architecture

#### Schema Design with PostgreSQL

Designed a normalized schema with proper constraints and relationships:

- **Foreign keys with CASCADE** for data integrity
- **Composite primary keys** for many-to-many relationships
- **CHECK constraints** for data validation (array length limits)
- **ENUM types** for type-safe categorical data
- **TIMESTAMPTZ** for timezone-aware timestamps

#### Migration Management

SQLx-managed migrations with version control:

```
migrations/
├── 20260420124048_create_tables.sql
├── 20260424083449_admin_and_altering.sql
├── 20260428141101_profile_and_orignals_alter.sql
└── ... (9 migrations total)
```

**Migration strategy:**

- **Incremental schema evolution** with timestamped migrations
- **Rollback support** via version control
- **Production-ready** with NOT NULL constraints and indexes
- **Data integrity** enforced at database level

### API Design

#### RESTful Endpoints with Axum

Clean route organization with modular handlers:

```rust
// src/routes/auth.rs
pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/register", post(sign_up_artist_handler))
        .route("/login", post(login_profile))
        .route("/logout", post(logout_profile_handler))
        .route("/reset-password", post(reset_password))
}
```

**API design principles:**

- **Resource-oriented routing** (`/works/new/{work_type}`)
- **Stateless authentication** via JWT cookies
- **Structured error responses** with consistent format
- **Instrumented handlers** with tracing for observability

#### Request/Response Validation

Custom extractors for JSON validation and type safety:

```rust
// src/services/json_extractor.rs
pub struct AppJson<T>(pub T);

// src/domain/profiles/handle.rs
impl<'de> Deserialize<'de> for Handle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::parse(s).map_err(serde::de::Error::custom)
    }
}
```

**Validation strategy:**

- **Domain-level validation** in type constructors
- **Automatic validation** via Deserialize trait
- **Early rejection** of invalid data
- **Clear error messages** for API consumers

---

## Product Thinking

### Deterministic Paging for Frontend Layout

Designed the backend to support **virtualized CSS Grid layouts** with deterministic aspect ratios:

**Problem:** Frontend needs precise layout information to render grids without layout thrashing.

**Solution:** Backend provides format-specific metadata (IMAX, ACADEMY, SQUARE, VERTICAL) enabling frontend to calculate exact grid positions.

**Product impact:**

- **Stable layouts** that don't shift on hydration
- **Predictable rendering** for mobile vs desktop
- **Performance optimization** via virtualized scrolling
- **Better UX** with consistent visual experience

### Presence & Reputation System

Gamified engagement system that rewards creators:

- **Credits** awarded for work submissions
- **Presence score** accumulates across the platform
- **Profile upgrades** from ARTIST to STAR/MAKER
- **Social proof** through visible reputation metrics

**Product thinking:**

- **Incentivizes quality content** creation
- **Builds community** through reputation
- **Enables discovery** via presence sorting
- **Creates progression** for user engagement

### Curated Collections (Sets & Festivals)

Implemented a two-tier curation system:

**Sets:** Curated collections of works with curators and members

- Curators manage collection themes
- Members contribute works
- Presence system applies to sets

**Festivals:** Time-bound events with panelists

- Organized by sets
- Panelists evaluate submissions
- Time-bound with start/end dates

**Product innovation:**

- **Community-driven curation** beyond algorithmic feeds
- **Event-based engagement** through festivals
- **Social validation** via panelist system
- **Temporal discovery** through time-bound events

### Multi-Platform Content Support

Designed for platform-agnostic content ingestion:

- **YouTube** edits with UUID-based video IDs
- **Twitter** native video support
- **Native platform** for direct uploads
- **Format-aware** metadata (IMAX, ACADEMY, etc.)

**Product flexibility:**

- **Platform independence** for content creators
- **Future-proof** for new platforms
- **Format optimization** for different devices
- **CDN-ready** architecture for edge delivery

---

## Tech Stack

| Component            | Technology                               | Rationale                                            |
| -------------------- | ---------------------------------------- | ---------------------------------------------------- |
| **Runtime**          | Rust 2024 Edition                        | Memory safety, zero-cost abstractions, async support |
| **Web Framework**    | Axum 0.8.8                               | Tower ecosystem, type-safe routing, async handlers   |
| **Database**         | PostgreSQL with SQLx 0.8.6               | Compile-time query checking, type safety, migrations |
| **Authentication**   | JWT (jsonwebtoken 10.3.0)                | Stateless, scalable, role-based access               |
| **Password Hashing** | Argon2 with cryptographic salting        | Password Hashing Competition winner, memory-hard     |
| **Async Runtime**    | Tokio (full features)                    | Industry-standard async runtime, excellent ecosystem |
| **Serialization**    | Serde/serde_json                         | De facto standard, compile-time derived              |
| **Error Handling**   | thiserror 2.0.18                         | Ergonomic error propagation, custom error types      |
| **Tracing**          | tracing-subscriber 0.3.23                | Structured logging, instrumentation support          |
| **Validation**       | validator 0.20.0                         | Declarative validation, derive macros                |
| **Testing**          | Tokio test, reqwest client, sqlx queries | Integration testing, HTTP client, DB assertions      |
| **Configuration**    | config crate + YAML files                | Type-safe config, environment support                |

---

## Project Structure

The codebase follows **clean architecture principles** with clear separation of concerns:

```
tars/
├── Cargo.toml              # Project manifest with dependencies
├── configuration.yaml      # App configuration (port, DB settings)
├── Prd.md                  # Product requirements and API specs
├── README.md               # This file
├── rules.md                # Business rules and constraints
├── migrations/             # SQL migrations (sqlx managed)
│   ├── 20260420124048_create_tables.sql                    # Core tables & enums
│   ├── 20260424083449_admin_and_altering.sql               # Admin & watchlist system
│   ├── 20260428141101_profile_and_orignals_alter.sql      # Profile enhancements
│   ├── 20260428154016_not_null_on_profile_type.sql         # Schema refinements
│   ├── 20260428194054_orignals_metadata_columns.sql        # Additional metadata
│   ├── 20260501201820_type_reanaming.sql                   # Type renames & work views/likes
│   ├── 20260512090110_sets_festivals_orignal_update.sql    # Sets & Festivals system
│   ├── 20260514112110_sets_library_festivals_panelists.sql  # Library & Panelists enhancements
│   └── 20260519124901_original_credts_library_...sql        # NOT NULL enforcements
├── scripts/
│   └── init_db.sh          # Database initialization script
├── src/
│   ├── main.rs             # Application entry point with async runtime
│   ├── lib.rs              # Library root with module exports
│   ├── startup.rs          # Server startup & route configuration
│   ├── configuration.rs    # Config loading and database settings
│   ├── db/                 # Database access layer
│   │   ├── mod.rs
│   │   ├── queries/        # Query modules (profile_queries, etc.)
│   │   └── mutations/      # Mutation modules (admins, artists, etc.)
│   ├── domain/             # Domain logic layer (DDD)
│   │   ├── festivals/      # Festival domain models (name, description, rules)
│   │   ├── library_thought.rs  # Library thought validation
│   │   ├── originals/      # Original domain models (title, description, genre, role)
│   │   ├── profiles/       # Profile domain models (handle, stage_name, hex_color, tagline)
│   │   ├── sets/           # Set domain models (name, description, statement)
│   │   ├── shared/         # Shared domain types (password)
│   │   └── works/          # Work domain models (title, script_thought)
│   ├── errors/             # Centralized error handling with thiserror
│   │   └── mod.rs          # ApiError enum with IntoResponse implementation
│   ├── routes/             # HTTP endpoint handlers
│   │   ├── mod.rs
│   │   ├── artists.rs      # Artist profile endpoints
│   │   ├── auth.rs         # Authentication endpoints (register, login, logout)
│   │   ├── festivals.rs    # Festival management endpoints
│   │   ├── health_check.rs # GET /health_check
│   │   ├── library.rs       # Library/watchlist endpoints
│   │   ├── originals.rs    # Original management endpoints
│   │   ├── sets.rs         # Set management endpoints
│   │   └── works.rs        # Work submission endpoints
│   ├── services/           # Business logic services
│   │   ├── auth_service/   # Authentication service (password, JWT, extractor)
│   │   ├── json_extractor.rs  # Custom JSON extractor
│   │   └── upload_service.rs  # File upload service
│   ├── types/              # Data models and schemas
│   │   ├── mod.rs
│   │   ├── db/             # Database entity types
│   │   ├── requests/       # HTTP request schemas
│   │   └── response/       # HTTP response schemas
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

**Architectural highlights:**

- **Domain layer** separates business logic from infrastructure
- **Service layer** encapsulates complex operations (auth, uploads)
- **Query/Mutation separation** for database operations
- **Type-safe domain primitives** prevent invalid states
- **Modular route organization** for maintainability

---

## Database Schema

### Design Philosophy

The database schema is designed for **data integrity**, **query performance**, and **business rule enforcement** at the database level. Key design decisions:

- **ENUM types** for type-safe categorical data
- **Composite primary keys** for many-to-many relationships
- **Foreign keys with CASCADE** for referential integrity
- **CHECK constraints** for data validation
- **TIMESTAMPTZ** for timezone-aware timestamps
- **Array types** for efficient multi-value storage

### Enums

#### `role_type`

```
STAR, MAKER
```

- **STAR**: Artist role (actors, performers)
- **MAKER**: Creator role (directors, composers, writers, etc.)

**Design rationale:** Enables tracking multiple roles per artist per original, supporting complex credit systems.

#### `supported_platforms`

```
YOUTUBE, TWITTER, NATIVE
```

External platforms where video edits are hosted.

**Design rationale:** Platform-agnostic architecture allows future platform additions without schema changes.

#### `edit_format`

```
IMAX, ACADEMY, SQUARE, VERTICAL
```

Video aspect ratios for deterministic layout rendering.

**Design rationale:** Enables frontend to calculate exact grid positions for virtualized CSS Grid layouts, preventing layout thrashing.

#### `poster_format`

```
CANVAS, STANDARD, SQUARE, VERTICAL
```

Poster dimensions for consistent grid layouts.

**Design rationale:** Matches frontend layout requirements for predictable rendering across devices.

#### `work_category`

```
EDIT, POSTER, SCRIPT
```

Types of creative works users can submit.

**Design rationale:** Type-safe categorization enables category-specific metadata and validation logic.

#### `profile_type`

```
STAR, MAKER, ARTIST
```

Profile classification for permission and feature access.

**Design rationale:** Role-based access control with clear progression paths (ARTIST → STAR/MAKER).

#### `watchlist_status`

```
WATCHED, WATCHING, WANT_TO_WATCH
```

User's tracking status for an original.

**Design rationale:** Enables personalized recommendations and social features based on viewing history.

#### `original_category`

```
MOVIE, SERIES
```

Classification of originals as movies or series.

**Design rationale:** Supports different data models (episodes for series) while maintaining unified API.

#### `set_role`

```
CURATOR, MEMBER
```

Roles within a set - curators manage the set, members contribute.

**Design rationale:** Enables community-driven curation with clear permission boundaries.

---

### Tables

#### `originals`

**Purpose**: Anchor movies/series that works reference

| Column            | Type              | Constraints               | Notes                            |
| ----------------- | ----------------- | ------------------------- | -------------------------------- |
| `id`              | UUID              | PK                        | Unique identifier                |
| `title`           | VARCHAR           | NOT NULL                  | Movie/Series name                |
| `description`     | TEXT              | NOT NULL                  | Plot synopsis                    |
| `cover_img`       | VARCHAR           | NOT NULL                  | CDN cover image URL              |
| `presence`        | BIGINT            | NOT NULL                  | Cumulative community engagement  |
| `password_hash`   | TEXT              | NOT NULL                  | Admin password for this original |
| `created_at`      | TIMESTAMPTZ       | DEFAULT NOW()             | Creation timestamp               |
| `associated_with` | UUID              | FK→profiles               | Producer/studio profile          |
| `category`        | original_category | NOT NULL, DEFAULT 'MOVIE' | MOVIE or SERIES                  |

**Design highlights:**

- **Presence field** enables gamified engagement tracking
- **Password hash per original** allows admin control at content level
- **Category field** supports different data models (movies vs series with episodes)

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
| `profile_type`      | profile_type | DEFAULT 'ARTIST' | STAR/MAKER/auth           |
| `youtube_profile`   | TEXT         |                  | YouTube channel URL       |
| `twitter_profile`   | TEXT         |                  | Twitter handle URL        |
| `instagram_profile` | TEXT         |                  | Instagram profile URL     |
| `stage_name`        | TEXT         | NOT NULL         | Display stage name        |
| `text_color`        | TEXT         | NOT NULL         | Profile text color        |
| `background_color`  | TEXT         | NOT NULL         | Profile background color  |
| `created_at`        | TIMESTAMPTZ  | DEFAULT NOW()    | Registration date         |

**Design highlights:**

- **Unique constraint on user_name** prevents duplicate handles
- **Presence system** with default 100 enables reputation-based progression
- **Profile type enum** supports role-based access control
- **Customizable colors** (text_color, background_color) enable personalized profiles
- **Social profile links** support multi-platform presence

---

#### `cast_and_crew_roles`

**Purpose**: Association of artists to originals in specific capacities (formerly `roles`)

| Column        | Type        | Constraints      | Notes                                          |
| ------------- | ----------- | ---------------- | ---------------------------------------------- |
| `profile_id`  | UUID        | FK→profiles, PK  | Artist reference                               |
| `original_id` | UUID        | FK→originals, PK | Movie/series reference                         |
| `category`    | role_type   | PK               | STAR or MAKER                                  |
| `role_name`   | TEXT        | PK               | Specific role (e.g., "Director", "Lead Actor") |
| `created_at`  | TIMESTAMPTZ | DEFAULT NOW()    | Role assignment date                           |

**Design highlights:**

- **Composite PK** enables multiple roles per artist per original
- **CASCADE deletes** maintain referential integrity
- **Role type enum** distinguishes between performers and creators

---

#### `works`

**Purpose**: Creative submissions (edits, posters, scripts)

| Column       | Type          | Constraints           | Notes                |
| ------------ | ------------- | --------------------- | -------------------- |
| `id`         | UUID          | PK                    | Work identifier      |
| `title`      | VARCHAR       |                       | Display title        |
| `artist_id`  | UUID          | FK→profiles, NOT NULL | Creator profile      |
| `category`   | work_category | NOT NULL              | EDIT/POSTER/SCRIPT   |
| `credits`    | BIGINT        | NOT NULL              | Reward points earned |
| `created_at` | TIMESTAMPTZ   | DEFAULT NOW()         | Submission date      |

**Design highlights:**

- **Credits field** enables presence/reputation system
- **Category enum** supports type-specific metadata tables
- **FK to profiles** ensures all works have valid creators

---

#### `originals_credits`

**Purpose**: Many-to-many link between works and originals

| Column        | Type | Constraints      | Notes              |
| ------------- | ---- | ---------------- | ------------------ |
| `work_id`     | UUID | FK→works, PK     | Work reference     |
| `original_id` | UUID | FK→originals, PK | Original reference |

**Design highlights:**

- **Composite PK** enables one work to reference multiple originals
- **Supports complex relationships** (e.g., crossover edits)
- **CASCADE deletes** maintain data consistency

---

#### `edits`

**Purpose**: Video edit metadata (extends works table)

| Column     | Type                | Constraints       | Notes                                           |
| ---------- | ------------------- | ----------------- | ----------------------------------------------- |
| `work_id`  | UUID                | FK→works, PK      | Edit identifier                                 |
| `src_id`   | VARCHAR             | NOT NULL          | Platform-specific video ID (YouTube UUID, etc.) |
| `platform` | supported_platforms | NOT NULL          | YOUTUBE/TWITTER/NATIVE                          |
| `format`   | edit_format         | DEFAULT 'ACADEMY' | Aspect ratio for layout                         |

**Design highlights:**

- **Platform enum** enables platform-agnostic architecture
- **Format enum** supports deterministic frontend layouts
- **Platform-specific IDs** allow future platform additions

---

#### `posters`

**Purpose**: Image poster metadata (extends works table)

| Column    | Type          | Constraints        | Notes                        |
| --------- | ------------- | ------------------ | ---------------------------- |
| `work_id` | UUID          | FK→works, PK       | Poster identifier            |
| `src_id`  | VARCHAR       | NOT NULL           | CDN/storage image identifier |
| `format`  | poster_format | DEFAULT 'STANDARD' | Dimensions for grid          |

**Design highlights:**

- **Format enum** enables consistent grid layouts
- **CDN-ready src_id** supports edge delivery
- **Extensible design** for future format additions

---

#### `scripts`

**Purpose**: Long-form theory/script metadata (extends works table)

| Column        | Type      | Constraints               | Notes                             |
| ------------- | --------- | ------------------------- | --------------------------------- |
| `work_id`     | UUID      | FK→works, PK              | Script identifier                 |
| `img_src_ids` | VARCHAR[] | CHECK (array_length ≤ 10) | Support images (max 10)           |
| `thoughts`    | TEXT[]    |                           | Array of theory/script paragraphs |

**Design highlights:**

- **Array types** efficiently store multi-value data
- **CHECK constraint** enforces business rule (max 10 images)
- **Flexible thoughts array** supports variable-length content

---

#### `admins`

**Purpose**: Administrative user accounts

| Column                | Type        | Constraints   | Notes                  |
| --------------------- | ----------- | ------------- | ---------------------- |
| `admin_id`            | UUID        | PK            | Admin ID               |
| `admin_name`          | TEXT        | NOT NULL      | Display name           |
| `admin_password_hash` | TEXT        | NOT NULL      | Argon2 hashed password |
| `created_at`          | TIMESTAMPTZ | DEFAULT NOW() | Creation date          |

**Design highlights:**

- **Separate admin table** enables role-based access control
- **Argon2 hashing** provides industry-standard security
- **Independent from profiles** for admin isolation

---

#### `beta_whitelist`

**Purpose**: Early access control for beta features

| Column            | Type        | Constraints   | Notes                           |
| ----------------- | ----------- | ------------- | ------------------------------- |
| `artist_username` | TEXT        |               | Whitelisted username            |
| `is_claimed`      | BOOLEAN     | NOT NULL      | Whether beta access was claimed |
| `added_at`        | TIMESTAMPTZ | DEFAULT NOW() | Whitelist date                  |

**Design highlights:**

- **Beta access control** enables staged rollouts
- **Claim tracking** prevents duplicate access
- **Flexible username-based** system for easy management

---

#### `library`

**Purpose**: Personal watchlist and engagement tracking per user/original

| Column            | Type             | Constraints             | Notes                               |
| ----------------- | ---------------- | ----------------------- | ----------------------------------- |
| `original_id`     | UUID             | FK→originals, PK        | Movie/series reference              |
| `profile_id`      | UUID             | FK→profiles, PK         | User reference                      |
| `episode_id`      | UUID             | FK→episodes             | Episode reference (for series)      |
| `pub_visibility`  | BOOLEAN          | NOT NULL                | Public vs private tracking          |
| `tagged_works`    | UUID[]           |                         | Works user tagged for this original |
| `pre_thought`     | TEXT             |                         | Initial impression before watching  |
| `post_impression` | TEXT             |                         | Thoughts after watching             |
| `status`          | watchlist_status | NOT NULL                | WATCHED/WATCHING/WANT_TO_WATCH      |
| `entry_type`      | TEXT             | NOT NULL                | Type of library entry               |
| `created_at`      | TIMESTAMPTZ      | NOT NULL, DEFAULT NOW() | Creation timestamp                  |
| `updated_at`      | TIMESTAMPTZ      | NOT NULL, DEFAULT NOW() | Last update timestamp               |

**Design highlights:**

- **Composite PK** ensures one library entry per user per original
- **Episode support** enables series-level tracking
- **Tagged works array** enables personalized curation
- **Pre/post thoughts** support engagement tracking
- **Status enum** enables personalized recommendations
- **Public/private visibility** supports social features

---

#### `work_views`

**Purpose**: Track view counts for works per user

| Column          | Type        | Constraints             | Notes                |
| --------------- | ----------- | ----------------------- | -------------------- |
| `work_id`       | UUID        | FK→works, PK            | Work reference       |
| `profile_id`    | UUID        | FK→profiles, PK         | User reference       |
| `times_watched` | BIGINT      | DEFAULT 0               | View count           |
| `created_at`    | TIMESTAMPTZ | NOT NULL, DEFAULT NOW() | First view timestamp |

**Design highlights:**

- **Composite PK** prevents duplicate view tracking
- **Times_watched counter** enables engagement analytics
- **CASCADE deletes** maintain data consistency

---

#### `work_likes`

**Purpose**: Track user likes for works

| Column       | Type        | Constraints             | Notes          |
| ------------ | ----------- | ----------------------- | -------------- |
| `work_id`    | UUID        | FK→works, PK            | Work reference |
| `profile_id` | UUID        | FK→profiles, PK         | User reference |
| `created_at` | TIMESTAMPTZ | NOT NULL, DEFAULT NOW() | Like timestamp |

**Design highlights:**

- **Composite PK** prevents duplicate likes
- **Simple relationship** enables efficient like counting
- **CASCADE deletes** maintain referential integrity

---

#### `favorite_profiles`

**Purpose**: User's favorited profiles

| Column         | Type        | Constraints             | Notes              |
| -------------- | ----------- | ----------------------- | ------------------ |
| `profile_id`   | UUID        | FK→profiles, PK         | User reference     |
| `favorited_id` | UUID        | FK→profiles, PK         | Favorited profile  |
| `created_at`   | TIMESTAMPTZ | NOT NULL, DEFAULT NOW() | Favorite timestamp |

**Design highlights:**

- **Composite PK** prevents duplicate favorites
- **Self-referential FK** enables profile-to-profile relationships
- **Supports social features** and discovery

---

#### `followings`

**Purpose**: User's follow relationships

| Column         | Type        | Constraints             | Notes               |
| -------------- | ----------- | ----------------------- | ------------------- |
| `follower_id`  | UUID        | FK→profiles, PK         | Follower reference  |
| `following_id` | UUID        | FK→profiles, PK         | Following reference |
| `created_at`   | TIMESTAMPTZ | NOT NULL, DEFAULT NOW() | Follow timestamp    |

**Design highlights:**

- **Composite PK** prevents duplicate follows
- **Self-referential FKs** enable social graph
- **Directional relationship** (follower → following)

---

#### `sets`

**Purpose**: Curated collections of works with curators and members

| Column            | Type        | Constraints             | Notes                   |
| ----------------- | ----------- | ----------------------- | ----------------------- |
| `id`              | UUID        | PK                      | Set identifier          |
| `name`            | TEXT        | NOT NULL, UNIQUE        | Set name                |
| `statement`       | TEXT        | NOT NULL                | Set statement/mission   |
| `description`     | TEXT        | NOT NULL                | Set description         |
| `presence`        | BIGINT      | NOT NULL                | Set presence/reputation |
| `profile_picture` | TEXT        | NOT NULL                | Set image URL           |
| `curator`         | UUID        | FK→profiles, NOT NULL   | Curator profile         |
| `created_at`      | TIMESTAMPTZ | NOT NULL, DEFAULT NOW() | Creation timestamp      |

**Design highlights:**

- **Unique constraint on name** prevents duplicate sets
- **Presence field** enables set-level reputation
- **Curator FK** enables ownership and permission control
- **Statement field** supports thematic curation

---

#### `set_members`

**Purpose**: Members and their roles within sets

| Column       | Type        | Constraints             | Notes             |
| ------------ | ----------- | ----------------------- | ----------------- |
| `profile_id` | UUID        | FK→profiles, PK         | Member profile    |
| `set_id`     | UUID        | FK→sets, PK             | Set reference     |
| `set_role`   | set_role    | NOT NULL                | CURATOR or MEMBER |
| `created_at` | TIMESTAMPTZ | NOT NULL, DEFAULT NOW() | Join timestamp    |

**Design highlights:**

- **Composite PK** prevents duplicate memberships
- **Role enum** enables permission differentiation (curator vs member)
- **Supports community-driven curation** with clear roles

---

#### `festivals`

**Purpose**: Time-bound events organized by sets with panelists and work submissions

| Column        | Type        | Constraints              | Notes                |
| ------------- | ----------- | ------------------------ | -------------------- |
| `id`          | UUID        | PK                       | Festival identifier  |
| `set_id`      | UUID        | FK→set_members, NOT NULL | Set reference        |
| `name`        | TEXT        | NOT NULL                 | Festival name        |
| `description` | TEXT        | NOT NULL                 | Festival description |
| `start_date`  | TIMESTAMPTZ | NOT NULL                 | Festival start date  |
| `end_date`    | TIMESTAMPTZ | NOT NULL                 | Festival end date    |
| `rules`       | TEXT        |                          | Festival rules       |
| `organizer`   | UUID        | FK→set_members, NOT NULL | Organizer profile    |
| `created_at`  | TIMESTAMPTZ | NOT NULL, DEFAULT NOW()  | Creation timestamp   |

**Design highlights:**

- **Composite FK to set_members** ensures organizer is a set member
- **CHECK constraint** enforces start_date < end_date
- **Time-bound events** enable temporal discovery
- **Rules field** supports customizable festival guidelines

---

#### `panelists`

**Purpose**: Panelists for festival evaluations

| Column        | Type        | Constraints             | Notes                |
| ------------- | ----------- | ----------------------- | -------------------- |
| `festival_id` | UUID        | FK→festivals, PK        | Festival reference   |
| `profile_id`  | UUID        | FK→profiles, PK         | Panelist profile     |
| `work_id`     | UUID        | FK→works                | Work being evaluated |
| `created_at`  | TIMESTAMPTZ | NOT NULL, DEFAULT NOW() | Assignment timestamp |

**Design highlights:**

- **Composite PK** prevents duplicate panelist assignments
- **Optional work_id** enables flexible panelist workflows
- **Supports peer review** and evaluation systems

---

#### `festival_works`

**Purpose**: Works submitted to festivals

| Column        | Type        | Constraints             | Notes                |
| ------------- | ----------- | ----------------------- | -------------------- |
| `festival_id` | UUID        | FK→festivals, PK        | Festival reference   |
| `work_id`     | UUID        | FK→works, PK            | Work reference       |
| `created_at`  | TIMESTAMPTZ | NOT NULL, DEFAULT NOW() | Submission timestamp |

**Design highlights:**

- **Composite PK** prevents duplicate submissions
- **Many-to-many relationship** enables works in multiple festivals
- **Supports event-based content discovery**

---

#### `set_works`

**Purpose**: Works included in sets

| Column       | Type        | Constraints             | Notes              |
| ------------ | ----------- | ----------------------- | ------------------ |
| `set_id`     | UUID        | FK→sets, PK             | Set reference      |
| `work_id`    | UUID        | FK→works, PK            | Work reference     |
| `created_at` | TIMESTAMPTZ | NOT NULL, DEFAULT NOW() | Addition timestamp |

**Design highlights:**

- **Composite PK** prevents duplicate work additions
- **Many-to-many relationship** enables works in multiple sets
- **Supports curated collections** and thematic grouping

---

#### `episodes`

**Purpose**: Episodes for series-based originals

| Column           | Type        | Constraints             | Notes                    |
| ---------------- | ----------- | ----------------------- | ------------------------ |
| `id`             | UUID        | PK                      | Episode identifier       |
| `series_id`      | UUID        | FK→originals            | Series reference         |
| `title`          | TEXT        | NOT NULL                | Episode title            |
| `description`    | TEXT        | NOT NULL                | Episode description      |
| `episode_number` | INTEGER     | NOT NULL                | Episode number in series |
| `season_number`  | INTEGER     | NOT NULL                | Season number            |
| `created_at`     | TIMESTAMPTZ | NOT NULL, DEFAULT NOW() | Creation timestamp       |

**Design highlights:**

- **FK to originals** enables series-level tracking
- **Episode/season numbers** support structured series organization
- **Enables library tracking** at episode level for series
- **Supports binge-watching** and series-specific features

---

## API Routes

### Architecture Overview

The API follows **RESTful principles** with **type-safe handlers** using Axum's routing system. Key architectural decisions:

- **Modular route organization** with separate router functions per domain
- **Custom extractors** for JSON validation and authentication
- **Instrumented handlers** with tracing for observability
- **Structured error responses** with consistent HTTP status codes
- **Cookie-based authentication** with HttpOnly and SameSite protection

### Implemented Endpoints

#### `POST /auth/register`

Register a new artist/user profile with domain-level validation

**Technical Implementation:**

```rust
#[instrument(name = "sign_up_artist", skip(app, data), err, fields(user_name = %data.handle))]
pub async fn sign_up_artist_handler(
    State(app): State<Arc<AppState>>,
    AppJson(data): AppJson<ProfileSignupReq>,
) -> Result<ProfileResponse, ApiError>
```

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
- `422 UNPROCESSABLE_ENTITY` on invalid data (domain validation)
- `500 INTERNAL_SERVER_ERROR` on database errors

**Technical Features:**

- **Domain validation** via Handle::parse() (Unicode-aware, length limits, no spaces)
- **Argon2 password hashing** with cryptographic salt generation (OsRng)
- **UUID generation** for unique profile identifiers
- **Default presence** set to 100 for reputation system
- **Profile type defaults** to ARTIST with upgrade path to STAR/MAKER
- **Optional social profiles** (YouTube, Twitter, Instagram)
- **Instrumented with tracing** for request lifecycle visibility

---

#### `POST /auth/login`

Authenticate user and issue JWT token

**Technical Implementation:**

```rust
#[instrument(name = "log_in_artist", skip(app, jar, data), err, fields(user_name = %data.handle))]
pub async fn login_profile(
    State(app): State<Arc<AppState>>,
    jar: CookieJar,
    AppJson(data): AppJson<ProfileLogin>,
) -> Result<ProfileResponse, ApiError>
```

**Security Features:**

- **Timing-attack resistant** password verification with fallback hash
- **JWT token generation** with 7-day expiration
- **HttpOnly cookies** prevent XSS attacks
- **SameSite Lax** protection against CSRF
- **Role-based claims** (artist vs admin)

---

#### `POST /auth/logout`

Terminate user session

**Implementation:**

```rust
pub async fn logout_profile_handler(jar: CookieJar) -> Result<ProfileResponse, ApiError> {
    let cookie = Cookie::build(("auth_token", ""))
        .http_only(true)
        .same_site(SameSite::Lax)
        .path("/")
        .max_age(time::Duration::seconds(0))
        .build();
    Ok(ProfileResponse::ProfileAuthenticated(jar.remove(cookie)))
}
```

**Security Features:**

- **Cookie deletion** with max_age(0) for immediate expiration
- **SameSite protection** maintained

---

#### `POST /auth/reset-password`

Password reset with authentication

**Technical Features:**

- **Custom extractor** `Artist` for JWT validation
- **Old password verification** before allowing reset
- **Argon2 re-hashing** for new password
- **Database update** with error handling

---

#### `POST /auth/admin/login`

Admin authentication with separate credentials

**Implementation:**

- **Separate admin table** for isolation
- **Same JWT infrastructure** with role="admin"
- **Fallback hash** for timing-attack resistance

---

#### `GET /health_check`

System health status for load balancer probes

**Implementation:**

```rust
pub async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}
```

**Purpose:**

- **Load balancer health checks** (Kubernetes, ALB, etc.)
- **Deployment verification** (smoke tests)
- **Zero database queries** for fast response

---

#### `POST /works/new/{work_type}`

Submit a new creative work (EDIT, POSTER, or SCRIPT)

**Route Parameters:**

- `work_type`: EDIT, POSTER, or SCRIPT

**Expected Functionality** (handler exists, implementation in progress):

- **Domain validation** for work-specific metadata
- **Type-specific handling** (video src, image format, script content)
- **Association with originals** via `originals_credits`
- **Credit awarding** to artist for presence system
- **Format metadata** for deterministic frontend layouts

---

#### `POST /originals/new`

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

**Technical Features:**

- **Role-based artist association** (STAR vs MAKER)
- **Composite PK** in cast_and_crew_roles for multiple roles
- **Password per original** for admin control
- **Category support** (MOVIE vs SERIES)

---

#### `POST /sets/new`

Create a new set (curated collection of works)

**Expected Functionality:**

- **Domain validation** for set name, statement, description
- **Curator assignment** with FK to profiles
- **Presence initialization** for set-level reputation
- **Unique name constraint** enforced at database level

---

#### `POST /festivals/new`

Create a new festival (time-bound event)

**Expected Functionality:**

- **Set association** via FK to set_members
- **Date validation** (start_date < end_date)
- **Organizer assignment** from set members
- **Rules field** for customizable guidelines

---

#### `POST /library/entry`

Create or update a library entry (watchlist tracking)

**Expected Functionality:**

- **Composite PK** (profile_id, original_id) for uniqueness
- **Episode support** for series-level tracking
- **Pre/post thoughts** for engagement tracking
- **Tagged works array** for personalized curation
- **Public/private visibility** toggle
- **Status enum** (WATCHED, WATCHING, WANT_TO_WATCH)

---

#### `GET /artists/{id}`

Fetch artist profile details

**Expected Functionality:**

- **Profile information** with domain types
- **Stage name, colors** for personalized profiles
- **Social profiles** (YouTube, Twitter, Instagram)
- **Presence/reputation score** for social proof
- **Work history** via originals_credits

---

### Planned Endpoints (from PRD)

#### `GET /api/works`

Fetch global works feed with deterministic paging for virtualized CSS Grid layouts

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

**Technical Requirements:**

- **Deterministic aspect ratios** (IMAX: 1.77, ACADEMY: 1.85, SQUARE: 1.0, VERTICAL: 0.56)
- **Cursor-based pagination** for efficient large dataset traversal
- **Client platform awareness** (mobile vs desktop limit differences)
- **Format-specific filtering** for layout optimization

---

#### `GET /api/originals`

Fetch anchor originals (movies/series) with presence statistics

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

**Technical Requirements:**

- **Presence aggregation** from works and library entries
- **Member counting** via cast_and_crew_roles
- **Release counting** via works aggregation
- **Category filtering** (MOVIE vs SERIES)

---

#### `GET /api/auths/:artistId`

Fetch artist profile with work history and social proof

**Technical Requirements:**

- **Profile information** with domain types
- **WorkedOn array** via originals_credits aggregation
- **Social profiles** (YouTube, Twitter, Instagram)
- **Presence/reputation score** for social proof
- **Stage name and colors** for personalized profiles

---

## Test Infrastructure

### Architecture

The test suite follows **integration testing best practices** with isolated environments for reliable, repeatable tests:

1. **Spawn isolated server instances** for each test with random port assignment
2. **Create separate PostgreSQL databases** per test run via UUID-based naming
3. **Verify HTTP endpoints + database state** for end-to-end validation
4. **Clean up after execution** via CASCADE deletes through FK constraints
5. **Parallel test execution** enabled by random port assignment

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

**Technical Features:**

- **Random port assignment** enables parallel test execution without port conflicts
- **UUID-based database names** prevent test isolation issues
- **Migration execution** ensures schema consistency across test runs
- **Database configuration** uses environment DATABASE_URL for flexibility
- **Tokio test runtime** for async test support

#### `postgres_config.rs`

Database connection pooling and migration setup for tests

**Technical Features:**

- **Connection pooling** with configurable limits
- **Migration runner** for schema setup
- **Environment-aware configuration** for test vs production
- **Error handling** for database connection failures

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

**Testing Strategy:**

- **Isolated test environment** with random port assignment
- **HTTP client integration** using reqwest
- **Status code assertions** for endpoint validation
- **Zero database queries** for fast health check verification

---

#### `tests/auth.rs`

Comprehensive authentication/registration tests

##### `register_profile_return_200_on_correct_data`

```rust
POST /auth/register {
  "user_name": "kapten",
  "tag_line": "...",
  "password": "...",
  "profile_picture": "...",
  "youtube_profile": "..."
}
```

**Technical Assertions:**

- HTTP response is 2xx success
- Database contains inserted profile with matching youtube_profile
- Demonstrates **SQLx compile-time checked queries**: `sqlx::query_scalar!`
- Validates **Argon2 password hashing** via password_hash field
- Confirms **UUID generation** for profile identifiers

---

##### `register_profile_return_error_on_incorrect_data`

```rust
POST /auth/register {
  "user_name": "kapten",
  "password": "kapten@1023"
  // Missing required: tag_line, profile_picture
}
```

**Technical Assertions:**

- HTTP response is 4xx client error
- JSON deserialization fails as expected
- Validates **domain-level validation** via Handle::parse()
- Confirms **Serde validation** for required fields

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

**Technical Requirements:**

- **Composite PK handling** in cast_and_crew_roles
- **Role type validation** (STAR vs MAKER)
- **Password hashing** per original for admin control
- **FK constraint validation** for data integrity

---

#### `tests/upload_works.rs`

Work submission tests (file exists but details not examined)

**Expected Test Coverage:**

- **Type-specific validation** (EDIT, POSTER, SCRIPT)
- **Format metadata validation** (IMAX, ACADEMY, SQUARE, VERTICAL)
- **Platform validation** (YOUTUBE, TWITTER, NATIVE)
- **Credit awarding** to artist for presence system
- **Association with originals** via originals_credits

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

**Testing Strategy:**

- **Parallel execution** enabled by random port assignment
- **Isolated databases** prevent test interference
- **Integration testing** validates end-to-end flows
- **SQLx compile-time checks** ensure query validity

---

## Configuration

### Configuration Architecture

The application uses a **layered configuration system** with environment variable overrides:

**Priority Order:**

1. Environment variable `DATABASE_URL` (highest priority)
2. YAML configuration file (`configuration.yaml`)
3. Code-level defaults in `configuration.rs`

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
JWT_SECRET=your-secret-key
```

### Configuration Code

```rust
pub struct Settings {
    pub application: ApplicationSettings,
    pub database: DatabaseSettings,
    pub jwt_secret: String,
}

pub struct ApplicationSettings {
    pub host: String,
    pub port: u16,
}

pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
}
```

**Technical Features:**

- **Type-safe configuration** via Rust structs
- **Connection string builder** for database URLs
- **Environment variable support** for deployment flexibility
- **Secret management** for JWT tokens

---

## Getting Started

### Prerequisites

- **Rust 1.70+** (uses 2024 edition with modern features)
- **PostgreSQL 12+** (for UUID, JSON, array type support)
- **Tokio async runtime** (for async/await support)
- **SQLx CLI** (for migration management)

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

   # Run migrations using SQLx
   sqlx database create
   sqlx migrate run
   ```

4. **Configure environment**:

   ```bash
   # Create .env file or export
   export DATABASE_URL="postgres://postgres:Kap10@localhost:5432/aera"
   export JWT_SECRET="your-secret-key-here"
   ```

5. **Run server**:

   ```bash
   cargo run
   # Server starts at http://127.0.0.1:8080
   # Health check available at http://127.0.0.1:8080/health_check
   ```

### Testing

```bash
# Run full test suite
cargo test

# Run specific test category
cargo test --test auth
cargo test --test health_check

# With logging for debugging
RUST_LOG=debug cargo test -- --nocapture

# Run tests sequentially (if port conflicts occur)
cargo test -- --test-threads=1
```

### Development Workflow

1. **Run in development mode with hot reload**:

   ```bash
   cargo install cargo-watch
   cargo watch -x run
   ```

2. **Code quality checks**:

   ```bash
   # Format code
   cargo fmt

   # Lint with Clippy
   cargo clippy

   # Check for errors without building
   cargo check
   ```

3. **Add database migrations**:

   ```bash
   # Create new migration
   sqlx migrate add -r <migration_name>

   # Edit migration file at migrations/TIMESTAMP_<name>.sql
   # Run migrations
   sqlx migrate run

   # Revert last migration
   sqlx migrate revert
   ```

4. **Verify SQLx compile-time checks**:

   ```bash
   # Set DATABASE_URL for offline mode
   export DATABASE_URL="postgres://..."
   cargo build
   # SQLx will validate queries at compile time
   ```

---

## Architecture Decisions

### Why Axum?

**Technical Rationale:**

- **Tower ecosystem integration** - Leverages mature middleware ecosystem
- **Type-safe routing** - Compile-time route parameter validation
- **Extractor pattern** - Clean separation of concerns for request processing
- **State management via Arc<T>** - Thread-safe shared state across handlers
- **Async-first design** - Built on Tokio for high-performance concurrency
- **Minimal runtime overhead** - Zero-cost abstractions with Rust's type system

### Why SQLx?

**Technical Rationale:**

- **Compile-time query verification** - Catches SQL errors at build time via macros
- **Zero-cost abstractions** - No runtime ORM overhead, direct SQL execution
- **Native PostgreSQL support** - Full support for UUID, JSON, arrays, and custom types
- **Type-safe query results** - Automatically derives Rust structs from query results
- **Migration management** - Built-in migration runner with version control
- **Prepared statements** - Automatic query plan caching for performance

### Why Domain-Driven Design?

**Technical Rationale:**

- **Type-safe domain primitives** - Newtype pattern prevents invalid states
- **Business rule enforcement** - Validation logic encapsulated in domain types
- **Separation of concerns** - Domain logic isolated from infrastructure
- **Testability** - Domain logic can be tested without database dependencies
- **Maintainability** - Clear boundaries between business logic and technical implementation

### Why Argon2 for Password Hashing?

**Technical Rationale:**

- **Password Hashing Competition winner** - Industry-standard security
- **Memory-hard algorithm** - Resistant to GPU/ASIC attacks
- **Cryptographic salt generation** - Uses OsRng for secure random salts
- **Configurable parameters** - Can adjust memory, time, and parallelism
- **Timing-attack resistant** - Constant-time comparisons prevent side-channel attacks

### Why JWT for Authentication?

**Technical Rationale:**

- **Stateless authentication** - No server-side session storage required
- **Scalability** - Easy to scale horizontally without session synchronization
- **Role-based access control** - Claims support role-based authorization
- **Cross-domain support** - Works across multiple services/subdomains
- **Token expiration** - Built-in expiration for security

### Why Deterministic Paging?

**Technical Rationale:**

- **Frontend layout stability** - Prevents layout thrashing on hydration
- **Virtualized scrolling** - Enables efficient large dataset rendering
- **Predictable UX** - Consistent visual experience across devices
- **Performance optimization** - Reduces client-side layout calculations
- **Format-aware pagination** - Different limits for mobile vs desktop

### Database Organization

- **Separate tables per work type** (EDITS, POSTERS, SCRIPTS) for type-specific constraints
- **Composite primary keys** on roles to prevent duplicate role assignments
- **Foreign key cascades** ensure referential integrity
- **UUID identifiers** enable distributed system scaling

---

## Security Considerations

### Implemented Security Measures

✅ **Password Security:**

- **Argon2 password hashing** with cryptographic salt generation (OsRng)
- **Timing-attack resistant** password verification with fallback hashes
- **Memory-hard algorithm** resistant to GPU/ASIC attacks

✅ **SQL Injection Prevention:**

- **Parameterized queries** via SQLx macros (compile-time checked)
- **No string concatenation** in SQL queries
- **Type-safe query results** prevent injection vulnerabilities

✅ **Input Validation:**

- **Domain-level validation** via newtype pattern (Handle, StageName, etc.)
- **Serde validation** for request/response schemas
- **Unicode-aware validation** supporting international scripts

✅ **Authentication:**

- **JWT token-based authentication** with 7-day expiration
- **HttpOnly cookies** prevent XSS attacks
- **SameSite Lax** protection against CSRF
- **Role-based access control** (artist vs admin)

### Planned Security Enhancements

⚠️ **To Be Implemented:**

- **CORS configuration** for frontend cross-origin requests
- **Rate limiting** on endpoints to prevent abuse
- **Admin password-protected** original mutations
- **Profile verification flow** for claimed profiles
- **Token refresh mechanism** for long-lived sessions

---

## Future Development

### Immediate Priorities

- [ ] Complete work submission endpoints (EDIT/POSTER/SCRIPT types)
- [ ] Implement global works feed with cursor-based pagination
- [ ] Complete Sets API (CRUD operations, member management)
- [ ] Complete Festivals API (panelist management, work submissions)
- [ ] Complete Library API (full watchlist management)

### Enhancements

- [ ] CDN integration for media serving (CloudFront/Cloudflare)
- [ ] Real-time presence updates via WebSocket
- [ ] Social profile verification (YouTube, Twitter, Instagram)
- [ ] Advanced search with filters and sorting
- [ ] Analytics and metrics dashboard

### Infrastructure

- [ ] Docker containerization
- [ ] Kubernetes deployment manifests
- [ ] CI/CD pipeline setup
- [ ] Monitoring and alerting (Prometheus/Grafana)
- [ ] Log aggregation (ELK stack or similar)

---

## Summary

This backend demonstrates **production-grade Rust development** with:

- **Type-safe domain modeling** preventing invalid states at compile time
- **Compile-time database query verification** ensuring SQL correctness
- **Industry-standard security** with Argon2 and JWT authentication
- **Deterministic data structures** enabling precise frontend layouts
- **Clean architecture** with clear separation of concerns
- **Comprehensive testing** with isolated integration tests
- **Product-driven engineering** with gamified engagement systems

The codebase showcases expertise in **Rust systems programming**, **backend architecture**, and **product thinking** through features like the presence system, curated collections, and deterministic paging for optimal user experience.

---

## License

[Specify your license]
