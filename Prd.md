# FrameHouse Backend PRD & Core Schema

## 1. Product Scope & Objective

This PRD defines the backend requirements to power **FrameHouse**, a highly cinematic, deterministic spatial platform where fans share edits, posters, and theories.

The backend must be built for **Performance** and **Deterministic Paging** because the frontend utilizes structured layout engines (virtualized CSS Grids/Clusters) that rely on precision payload structures.

## 2. Core Entities & Database Models

### 2.1 Profiles (Users / Artists)

- Regular users authenticate and build "Presence" over time by securing Credits.
- Once enough Presence is met, they upgrade to "Artist" status.

### 2.2 Works

Works are the assets. They break down perfectly into categories defining dimensions for the frontend:

- `Edit`/`video` (requires `platform` and `platformId` like YouTube UUID)
- `Poster` (requires `image`)
- `Script` (requires `image` context and long-form data)

### 2.3 Originals

The "Movie" or "Series" that acts as the focal anchor for works (e.g., _RRR_, _OG_).

---

## 3. API Route Specifications & JSON Schemas

### A. Works Engine

#### `GET /api/works` -- `GET /tars/works/user/

**Purpose:** Fetches global works for the home theatre layout. For mobile, it must populate deterministic layout rules (e.g., making sure IMAX slots get `aspectRatio: 1.77` edits contextually).
**Query Params:** `?page=1&limit=5&category=Edit&clientPlatform=mobile` (use `limit=12` for desktop clusters)
**Expected JSON Response:**

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

#### `POST /api/works`

**Purpose:** Submission of new artworks. Images and Videos must be processed asynchronously and served geographically closest via edge-CDN. Request must include the `aspectRatio` dynamically calculated by the client to inform backend tagging.

### B. Original (Theatres) Engine

#### `GET /api/originals`

**Purpose:** Fetches the anchored movies for the home navigation loops.
**Expected JSON Response:**

```json
{
  "success": true,
  "data": [
    {
      "id": "og-original",
      "title": "OG",
      "description": "They call him OG. Pawan Kalyan's most raw avatar.",
      "coverImage": "https://cdn.framehouse.com/covers/og.png",
      "stats": {
        "presence": 2480,
        "members": 1620,
        "releases": 18
      },
      "releaseDate": "2025"
    }
  ]
}
```

### C. Artist Vault & Presence Engine

#### `GET /api/auths/:artistId`

**Purpose:** Serves the 3D Profile Talent HUDs. Must return an array of `workedOn` records (origins) for flip-card validation.
**Expected JSON Response:**

```json
{
  "success": true,
  "data": {
    "id": "fh-001",
    "name": "Karthik G",
    "bio": "Cinematic visualist...",
    "presence": 1540,
    "works": 24,
    "image": "https://cdn.framehouse.com/auths/pic.jpg",
    "socials": {
      "instagram": "@karthik_g",
      "youtube": "KarthikGVisuals"
    },
    "workedOn": [
      { "id": "og-original", "title": "OG" },
      { "id": "rrr-original", "title": "RRR" }
    ]
  }
}
```

---

## 4. Systems, Performance & Reliability

1. **CDN Delivery & Edge Caching:**
   Because all images represent _art_, lossy compression should be minimized. Deliver WEBP/AVIF formatted outputs over an Edge CDN to strictly enforce fast Largest Contentful Paint (LCP) in theatre layouts.
2. **Deterministic Layout Generation (Optional Backend Overdrive):**
   Ideally, `/api/layout/mobile` explicitly provides grid placement sequences: `{ type: 'IMAX', work: {...} }, { type: 'Vertical', work: {...} }` rather than the frontend rolling the dice. This stabilizes layout thrashing upon hydration.
3. **Optimistic UI Updates:**
   The backend must support idempotency on `/credit` endpoints so the client app can confidently update local variables prior to receiving HTTP 200 validations without fear of duplicate credit allocations.
4. **Data Isolation (Security):**
   Since videos are constructed on the client, the backend must strictly validate the `platformId` format (e.g., alphanumeric regex for YouTube UUIDs, numeric for Twitter constraints) to prevent injection risks locally.
