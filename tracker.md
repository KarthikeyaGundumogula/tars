# FrameHouse Backend Progress Tracker

This document tracks the implementation status of the **TARS** (Rust/Axum) backend and its integration with the **Aera** frontend.

---

## 🏗️ 1. Core Infrastructure
| Feature | Status | TARS (Rust) | Frontend (Aera) | Notes |
| :--- | :---: | :--- | :--- | :--- |
| **Database Schema** | 🟢 | Migrations 1-13 applied | N/A | PostgreSQL + SQLx |
| **Auth System** | 🟢 | JWT auth + Argon2 hashing | Login UI ready | Extractor & cookie validation done |
| **CDN / S3 Integration**| 🔴 | Not Started | Presigned URL logic | Need generation logic in TARS |
| **Domain Validation** | 🟢 | 29 Domain types implemented | Form validation | Strict type safety + 139 passed tests |
| **XSS Defense** | 🟢 | Script tag rejection | React escaped | Rejects `<script>`, `<iframe>`, `javascript:` |

---

## 🎬 2. Originals Engine
| Feature | Status | TARS (Rust) | Frontend (Aera) | Notes |
| :--- | :---: | :--- | :--- | :--- |
| **Original Creation** | 🟢 | Handler + DB Atomic | Admin UI ready | Stars/Makers linkage active |
| **Fetch All Originals** | 🟡 | Handler exists | Using Mock data | Wire `/api/originals` |
| **Original Metadata** | 🟢 | Columns added | Header logic ready | Certification & duration added |
| **Stats calculation** | 🔴 | Not Started | Mock stats | presence, members, releases |

---

## 🖼️ 3. Works Engine (Artifacts)
| Feature | Status | TARS (Rust) | Frontend (Aera) | Notes |
| :--- | :---: | :--- | :--- | :--- |
| **POST Edit (Video)** | 🟡 | Basic handler | Step 5 SEAL ready | Needs proper artist_id linkage |
| **POST Poster (Image)**| 🟡 | Stub (UUID return) | Geometry step ready| Needs file storage logic |
| **POST Script (Text)** | 🟡 | Stub (UUID return) | Upload logic ready | Needs multi-image support |
| **Wall Post Lines** | 🟢 | Domain type + XSS protection | UI active | Max 500 chars, whitespace trimmed |
| **GET Works (Feed)** | 🔴 | Not Started | Cluster Builder (FE)| Requires pagination + clusters |

---

## 👤 4. Artist Vault & Presence
| Feature | Status | TARS (Rust) | Frontend (Aera) | Notes |
| :--- | :---: | :--- | :--- | :--- |
| **Artist Profiles** | 🟢 | Handler exists | UI Components ready | |
| **Presence Tracking** | 🔴 | Not Started | Mock presence | logic for credit-based rank |
| **WorkedOn Linkage** | 🟡 | Roles DB active | ArtistProfile HUD | Need JOIN query for profile |
| **Artist Search** | 🟢 | Handler exists | PersonSearchInput | |
| **Social Profiles** | 🟢 | Domain validation | UI active | Max 100 chars, no whitespace |

---

## 📒 5. Watchlist / Library & Ledger
| Feature | Status | TARS (Rust) | Frontend (Aera) | Notes |
| :--- | :---: | :--- | :--- | :--- |
| **CTE Snapshot Fix** | 🟢 | SQL CTE isolation fixed | Active | Correct peak_score recalculation |
| **Library Scores** | 🟢 | Welford algorithm active | Active | Preserves mean_surge & surge_spread |
| **Recommendation Notes**| 🟢 | Domain type + XSS protection | Active | Max 2,000 chars |
| **Fetch Watchlist** | 🔴 | Not Started | LedgerPage UI | |
| **Add to Watchlist** | 🔴 | Not Started | AddEntry modal | |

---

## 💬 6. Discussions & Sets Engine
| Feature | Status | TARS (Rust) | Frontend (Aera) | Notes |
| :--- | :---: | :--- | :--- | :--- |
| **Set Creation & Update** | 🟢 | Handlers + DB mutations | Active | Secured with `OwnedResourceOrAdmin<Set>` |
| **Festival Creation** | 🟢 | Handlers + Panelist linkage | Active | Refactored to `OwnedResourceOrAdmin<Set>` |
| **Discussion Posts** | 🟢 | Create, Update, Delete CRUD | Active | Secured with `OwnedResourceOrAdmin<DiscussionPost>` |
| **Discussion Comments** | 🟢 | Create, Update, Delete CRUD | Active | Secured with `OwnedResourceOrAdmin<DiscussionComment>` |
| **Emoji Reactions** | 🟢 | Domain type validation | Active | Unicode emoji range checks |

---

## 💎 7. Social & Credits
| Feature | Status | TARS (Rust) | Frontend (Aera) | Notes |
| :--- | :---: | :--- | :--- | :--- |
| **Give Credit** | 🔴 | Not Started | UI trigger exists | Need idempotency logic |
| **Activity Feed** | 🔴 | Not Started | Placeholder | |

---

## 🚀 Immediate Backend TODOs (TARS)
1. [x] **JWT Implementation**: Add middleware to protect `POST /api/works` and `POST /api/originals`.
2. [x] **WorkArtist Linkage**: In `create_new_work_handler`, replace `Uuid::new_v4()` with the authenticated user's `artist_id`.
3. [x] **Domain Validation Engine**: Implement 29 domain types with strict character, length, and Unicode emoji validation (`Emoji`, `WallPostLine`, `SocialProfile`, `FilmCertification`, `RoleName`, `PermissionName`, etc.).
4. [x] **Discussions & Sets Engine**: Complete Discussion Post and Comment CRUD routes (`create`, `update`, `delete`) with SQLx mutations and `OwnedResourceOrAdmin` security extractors.
5. [x] **XSS Defense-in-Depth**: Integrated executable script tag rejection (`<script>`, `<iframe>`, `javascript:`, `onerror=`, `onload=`) across content domain types.
6. [x] **Library & Surge Metrics**: Fixed CTE snapshot isolation bug in `delete_recommendation`, preserved score calculation metrics (`peak_score`, `mean_surge`, `surge_spread`).
7. [ ] **Originals Fetch**: Implement `GET /api/originals` with stats (joining `works` and `roles` counts).
8. [ ] **Cluster Builder**: Port `clusterBuilder.ts` logic to Rust for deterministic `/api/works` payload.

---

## 🛠️ Tools Used
- **Database**: PostgreSQL with `sqlx`
- **API Framework**: `axum`
- **Auth**: Argon2 password hashing + JWT extractor
- **Mock Sync**: `src/mock/` in Aera mirrors the PRD schemas.
