# FrameHouse Backend Progress Tracker

This document tracks the implementation status of the **TARS** (Rust/Axum) backend and its integration with the **Aera** frontend.

---

## 🏗️ 1. Core Infrastructure
| Feature | Status | TARS (Rust) | Frontend (Aera) | Notes |
| :--- | :---: | :--- | :--- | :--- |
| **Database Schema** | 🟢 | Migrations 1-5 applied | N/A | PostgreSQL + SQLx |
| **Auth System** | 🟡 | Password hashing done | Login UI ready | Need JWT issuing/validation |
| **CDN / S3 Integration**| 🔴 | Not Started | Presigned URL logic | Need generation logic in TARS |
| **Deterministic Layout** | 🔴 | Not Started | Engine exists (FE) | Move cluster logic to Rust |

---

## 🎬 2. Originals Engine
| Feature | Status | TARS (Rust) | Frontend (Aera) | Notes |
| :--- | :---: | :--- | :--- | :--- |
| **Original Creation** | 🟢 | Handler + DB Atomic | Admin UI ready | Stars/Makers linkage active |
| **Fetch All Originals** | 🟡 | Handler exists | Using Mock data | Wire `/api/originals` |
| **Original Metadata** | 🟢 | Columns added | Header logic ready | |
| **Stats calculation** | 🔴 | Not Started | Mock stats | presence, members, releases |

---

## 🖼️ 3. Works Engine (Artifacts)
| Feature | Status | TARS (Rust) | Frontend (Aera) | Notes |
| :--- | :---: | :--- | :--- | :--- |
| **POST Edit (Video)** | 🟡 | Basic handler | Step 5 SEAL ready | Needs proper artist_id linkage |
| **POST Poster (Image)**| 🟡 | Stub (UUID return) | Geometry step ready| Needs file storage logic |
| **POST Script (Text)** | 🟡 | Stub (UUID return) | Upload logic ready | Needs multi-image support |
| **GET Works (Feed)** | 🔴 | Not Started | Cluster Builder (FE)| Requires pagination + clusters |

---

## 👤 4. Artist Vault & Presence
| Feature | Status | TARS (Rust) | Frontend (Aera) | Notes |
| :--- | :---: | :--- | :--- | :--- |
| **Artist Profiles** | 🟢 | Handler exists | UI Components ready | |
| **Presence Tracking** | 🔴 | Not Started | Mock presence | logic for credit-based rank |
| **WorkedOn Linkage** | 🟡 | Roles DB active | ArtistProfile HUD | Need JOIN query for profile |
| **Artist Search** | 🟢 | Handler exists | PersonSearchInput | |

---

## 📒 5. Watchlist / Ledger
| Feature | Status | TARS (Rust) | Frontend (Aera) | Notes |
| :--- | :---: | :--- | :--- | :--- |
| **Fetch Watchlist** | 🔴 | Not Started | LedgerPage UI | |
| **Add to Watchlist** | 🔴 | Not Started | AddEntry modal | |
| **Update Entry** | 🔴 | Not Started | Edit thoughts logic | Hype vs After-thoughts |

---

## 💎 6. Social & Credits
| Feature | Status | TARS (Rust) | Frontend (Aera) | Notes |
| :--- | :---: | :--- | :--- | :--- |
| **Give Credit** | 🔴 | Not Started | UI trigger exists | Need idempotency logic |
| **Activity Feed** | 🔴 | Not Started | Placeholder | |

---

## 🚀 Immediate Backend TODOs (TARS)
1. [ ] **JWT Implementation**: Add middleware to protect `POST /api/works` and `POST /api/originals`.
2. [ ] **WorkArtist Linkage**: In `create_new_work_handler`, replace `Uuid::new_v4()` with the authenticated user's `artist_id`.
3. [ ] **Originals Fetch**: Implement `GET /api/originals` with stats (joining `works` and `roles` counts).
4. [ ] **Watchlist Engine**: Create `watchlist` table and basic CRUD handlers.
5. [ ] **Cluster Builder**: Port `clusterBuilder.ts` logic to Rust for deterministic `/api/works` payload.

---

## 🛠️ Tools Used
- **Database**: PostgreSQL with `sqlx`
- **API Framework**: `axum`
- **Auth**: Argon2 password hashing
- **Mock Sync**: `src/mock/` in Aera mirrors the PRD schemas.
