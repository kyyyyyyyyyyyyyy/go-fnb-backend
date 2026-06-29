# API Documentation — Go FnB Backend

## Base URL

```
http://localhost:8081
```

## Authentication

### JWT Token

Hampir semua endpoint (kecuali public) membutuhkan **Bearer Token** (JWT).

**Header:**
```
Authorization: Bearer <token>
```

### Claims dalam JWT

```json
{
  "sub": "uuid-user-id",
  "outlet_id": "uuid-outlet-id (opsional)",
  "exp": 1234567890
}
```

- `sub` — ID user (selalu ada)
- `outlet_id` — ID outlet yang dimiliki/diakses user (opsional, hanya ada jika user terdaftar di sebuah outlet)
- `exp` — expiry 24 jam

### Cara Mendapatkan Token

**Login:**
```
POST /login
{ "email": "...", "password": "..." }
→ { "success": true, "token": "eyJ..." }
```

**Google OAuth:**
```
GET /auth/google/callback?code=...
→ { "success": true, "token": "eyJ..." }
```

### outlet_id dari JWT

Untuk semua **protected endpoint** yang membutuhkan `outlet_id`, nilai tersebut **tidak perlu dikirim** di body URL maupun body request. `outlet_id` sudah otomatis dibaca dari JWT. Ini berlaku untuk endpoint:

- Produk (create, get by outlet, get by id, update, delete)
- Order (get by outlet, get by id, update, delete, update status)
- Kategori (create, get by outlet, get by id, update, delete)
- Table (create, get by outlet, get by id, update, delete, delete token)
- QR Code (create, get by outlet, get tables, regenerate slug)

---

## Response Format

Semua response mengikuti format:

### Success (200)
```json
{ "success": true, "message": "...", "data": { ... } }
```

### Success (201 Created)
```json
{ "success": true, "message": "..." }
```

### Error (400 / 401 / 404)
```json
{ "success": false, "message": "..." }
```

---

## Endpoints

### 1. Health Check
**Public**

| Method | Path | Description |
|--------|------|-------------|
| GET | `/health` | Cek server |

Response: `"OK"`

---

### 2. Autentikasi
**Public**

#### Register
```
POST /register
Content-Type: application/json

{
  "name": "string",
  "email": "string",
  "password": "string"
}

Response 200:
{ "success": true, "message": "Register berhasil" }
```

#### Login
```
POST /login
Content-Type: application/json

{
  "email": "string",
  "password": "string"
}

Response 200:
{ "success": true, "token": "eyJ..." }
```

#### Google OAuth Callback
```
GET /auth/google/callback?code=...

Response 200:
{ "success": true, "token": "eyJ..." }
```

---

### 3. Outlet
**Protected (Bearer Token)**

| Method | Path | Description |
|--------|------|-------------|
| POST | `/api/outlets` | Buat outlet baru |
| GET | `/api/outlets/me` | Ambil outlet milik user |
| GET | `/api/outlets/{id}` | Detail outlet by ID |
| PATCH | `/api/outlets/{id}` | Update outlet |
| DELETE | `/api/outlets/{id}` | Hapus outlet |

#### Create Outlet
```
POST /api/outlets
Authorization: Bearer <token>

{
  "name": "string",
  "address_line": "string",
  "city": "string",
  "province": "string",
  "postal_code": "string | null",
  "latitude": "number | null",
  "longitude": "number | null"
}
```

#### Get My Outlets
```
GET /api/outlets/me
Authorization: Bearer <token>
```

Response 200:
```json
{
  "success": true,
  "message": "My outlets fetched successfully",
  "data": [
    {
      "id": "uuid",
      "name": "string",
      "owner_id": "uuid",
      "address_line": "string",
      "city": "string",
      "province": "string",
      "postal_code": "string | null",
      "latitude": "number | null",
      "longitude": "number | null",
      "today_revenue": 0,
      "today_orders": 0
    }
  ]
}
```

#### Get Outlet By ID
```
GET /api/outlets/{id}
Authorization: Bearer <token>
```

#### Update Outlet
```
PATCH /api/outlets/{id}
Authorization: Bearer <token>

{
  "name": "string | null",
  "address_line": "string | null",
  "city": "string | null",
  "province": "string | null",
  "postal_code": "string | null",
  "latitude": "number | null",
  "longitude": "number | null"
}
```

#### Delete Outlet
```
DELETE /api/outlets/{id}
Authorization: Bearer <token>
```

---

### 4. Invite (Undangan)

| Method | Path | Auth | Description |
|--------|------|------|-------------|
| POST | `/api/invites` | Bearer | Buat undangan |
| GET | `/api/invites/validate/{token}` | Public | Validasi token undangan |
| POST | `/api/invites/use` | Public | Terima undangan & buat akun |
| DELETE | `/api/invites/{id}` | Bearer | Hapus undangan |

#### Create Invite
```
POST /api/invites
Authorization: Bearer <token>

{
  "outlet_id": "uuid",
  "role": "admin | cashier"
}

Response:
{
  "success": true,
  "message": "Invite created successfully",
  "data": { "url": "http://localhost:8080/api/invites/validate/{token}" }
}
```

#### Validate Invite
```
GET /api/invites/validate/{token}

Response:
{
  "success": true,
  "message": "Invite valid",
  "data": { ... invite details ... }
}
```

#### Use Invite (terima undangan)
```
POST /api/invites/use
Content-Type: application/json

{
  "token": "string",
  "name": "string",
  "email": "string",
  "password": "string"
}
```

#### Delete Invite
```
DELETE /api/invites/{id}
Authorization: Bearer <token>
```

---

### 5. Kategori
**Protected (Bearer Token)** — `outlet_id` diambil dari JWT

| Method | Path | Description |
|--------|------|-------------|
| POST | `/api/categories` | Buat kategori |
| GET | `/api/categories` | Semua kategori (outlet dari JWT) |
| GET | `/api/categories/{category_id}` | Detail kategori |
| PUT | `/api/categories/{category_id}` | Update kategori |
| DELETE | `/api/categories/{category_id}` | Hapus kategori |

#### Create Category
```
POST /api/categories
Authorization: Bearer <token>

{
  "name": "string"
}
```

#### Get Categories (by outlet dari JWT)
```
GET /api/categories
Authorization: Bearer <token>
```

#### Get Category By ID
```
GET /api/categories/{category_id}
Authorization: Bearer <token>
```

#### Update Category
```
PUT /api/categories/{category_id}
Authorization: Bearer <token>

{
  "name": "string"
}
```

#### Delete Category
```
DELETE /api/categories/{category_id}
Authorization: Bearer <token>
```

---

### 6. Produk
**Protected (Bearer Token)** — `outlet_id` diambil dari JWT

| Method | Path | Description |
|--------|------|-------------|
| POST | `/api/products` | Tambah produk |
| GET | `/api/products` | Semua produk (outlet dari JWT) |
| GET | `/api/products/{product_id}` | Detail produk |
| PATCH | `/api/products/{product_id}` | Update produk (partial) |
| PATCH | `/api/products/{product_id}/available` | Toggle ketersediaan produk |
| DELETE | `/api/products/{product_id}` | Hapus produk |

#### Create Product
```
POST /api/products
Authorization: Bearer <token>

{
  "name": "string",
  "capital_price": "int64",
  "tax": "int64",
  "profit": "int64",
  "image_url": "string",
  "category_ids": ["uuid", "uuid", ...]
}
```

`price` dihitung otomatis: `capital_price + tax + profit`.

#### Get Products
```
GET /api/products
Authorization: Bearer <token>
```

Response:
```json
{
  "success": true,
  "message": "products fetched successfully",
  "data": [
    {
      "id": "uuid",
      "name": "string",
      "capital_price": 0,
      "tax": 0,
      "profit": 0,
      "price": 0,
      "image_url": "string",
      "available": true,
      "categories": [ { "id": "uuid", "name": "string" } ]
    }
  ]
}
```

#### Get Product By ID
```
GET /api/products/{product_id}
Authorization: Bearer <token>
```

#### Update Product
```
PATCH /api/products/{product_id}
Authorization: Bearer <token>

{
  "name": "string | null",
  "capital_price": "int64 | null",
  "tax": "int64 | null",
  "profit": "int64 | null",
  "image_url": "string | null",
  "add_category_ids": ["uuid", ...] | null,
  "remove_category_ids": ["uuid", ...] | null
}
```

`price` dihitung ulang otomatis dari `capital_price + tax + profit` (nilai final setelah merge).

#### Toggle Available Product
```
PATCH /api/products/{product_id}/available
Authorization: Bearer <token>

{
  "available": true | false
}

Response:
{ "success": true, "message": "product availability updated successfully" }
```

#### Delete Product
```
DELETE /api/products/{product_id}
Authorization: Bearer <token>
```

---

### 7. QR Code
**Protected (Bearer Token)** — `outlet_id` diambil dari JWT

| Method | Path | Description |
|--------|------|-------------|
| POST | `/api/qrcodes` | Buat QR & attach table |
| POST | `/api/qrcodes/auto` | Buat table + QR sekali jalan |
| GET | `/api/qrcodes` | Semua QR (outlet dari JWT) |
| GET | `/api/qrcodes/{qr_id}/tables` | Table yg terattach ke QR |
| PATCH | `/api/qrcodes/{qr_id}/regenerate` | Regenerate slug QR |

#### Create QR
```
POST /api/qrcodes
Authorization: Bearer <token>

{
  "table_ids": ["uuid", "uuid", ...]
}
```

#### Create QR With Tables (otomatis buat table baru + QR)
```
POST /api/qrcodes/auto
Authorization: Bearer <token>

{
  "tables": [
    { "name": "string", "location": "string" },
    ...
  ]
}
```

#### Get QRs
```
GET /api/qrcodes
Authorization: Bearer <token>
```

Response:
```json
{
  "success": true,
  "message": "QRs fetched successfully",
  "data": [
    {
      "id": "uuid",
      "slug": "string",
      "table_ids": ["uuid", ...]
    }
  ]
}
```

#### Get QR Tables
```
GET /api/qrcodes/{qr_id}/tables
Authorization: Bearer <token>
```

#### Regenerate QR Slug
```
PATCH /api/qrcodes/{qr_id}/regenerate
Authorization: Bearer <token>
```

---

### 8. Scan QR & Select Table
**Public**

| Method | Path | Description |
|--------|------|-------------|
| GET | `/api/scan/{slug}` | Scan QR code (cari by slug) |
| POST | `/api/scan/select-table` | Pilih meja & dapatkan token |

#### Scan QR
```
GET /api/scan/{slug}
```

Response (single table):
```json
{
  "success": true,
  "message": "QR found",
  "data": {
    "qr_id": "uuid",
    "qr_type": "single",
    "outlet_id": "uuid",
    "table": { "id": "uuid", "name": "string" },
    "tables": null,
    "categories": [
      { "id": "uuid", "name": "string" }
    ],
    "products": [
      {
        "id": "uuid",
        "name": "string",
        "price": 0,
        "image_url": "string",
        "categories": [ { "id": "uuid", "name": "string" } ]
      }
    ]
  }
}
```

Response (multi table):
```json
{
  "success": true,
  "message": "QR found",
  "data": {
    "qr_id": "uuid",
    "qr_type": "multi",
    "outlet_id": "uuid",
    "table": null,
    "tables": [
      { "id": "uuid", "name": "string" }
    ],
    "categories": [
      { "id": "uuid", "name": "string" }
    ],
    "products": [
      {
        "id": "uuid",
        "name": "string",
        "price": 0,
        "image_url": "string",
        "categories": [ { "id": "uuid", "name": "string" } ]
      }
    ]
  }
}
```

Response (meja sudah digunakan — single table dengan token terisi):
```
Status: 400
{ "success": false, "message": "meja sudah digunakan" }
```

#### Select Table
```
POST /api/scan/select-table
Content-Type: application/json

{
  "table_id": "uuid"
}

Response:
{
  "success": true,
  "message": "Table selected",
  "data": "token-string"
}
```

---

### 9. Table
**Protected (Bearer Token)** — `outlet_id` diambil dari JWT

| Method | Path | Description |
|--------|------|-------------|
| POST | `/api/tables` | Tambah meja |
| GET | `/api/tables` | Semua meja (outlet dari JWT) |
| GET | `/api/tables/{id}` | Detail meja |
| PATCH | `/api/tables/{id}` | Update meja (partial) |
| DELETE | `/api/tables/{id}` | Hapus meja |
| DELETE | `/api/tables/{id}/token` | Hapus token meja (reset ke available) |

#### Create Table
```
POST /api/tables
Authorization: Bearer <token>

{
  "name": "string",
  "location": "string | null"
}
```

#### Get Tables
```
GET /api/tables
Authorization: Bearer <token>
```

Response:
```json
{
  "success": true,
  "message": "Tables fetched successfully",
  "data": [
    {
      "id": "uuid",
      "outlet_id": "uuid",
      "name": "string",
      "location": "string | null",
      "status": "string"
    }
  ]
}
```

#### Update Table
```
PATCH /api/tables/{id}
Authorization: Bearer <token>

{
  "name": "string | null",
  "location": "string | null",
  "status": "string | null"
}
```

#### Delete Table Token (reset status ke available)
```
DELETE /api/tables/{id}/token
Authorization: Bearer <token>
```

---

### 10. Order

| Method | Path | Auth | Description |
|--------|------|------|-------------|
| POST | `/api/orders/consume` | Public | Buat order (dari customer) |
| GET | `/api/orders` | Bearer | Semua order (outlet dari JWT) |
| GET | `/api/orders/{id}` | Bearer | Detail order + items |
| PATCH | `/api/orders/{id}` | Bearer | Update order (partial) |
| PATCH | `/api/orders/{id}/status` | Bearer | Update status order |
| DELETE | `/api/orders/{id}` | Bearer | Hapus order |

#### Create Order (Public — dari customer)
```
POST /api/orders/consume

{
  "order_name": "string",
  "order_type": "string",
  "table_id": "uuid",
  "outlet_id": "uuid",
  "notes": "string | null",
  "order_items": [
    {
      "product_id": "uuid",
      "qty": "int32",
      "notes": "string | null"
    }
  ]
}
```

Harga dihitung otomatis dari data produk (capital_price, tax, profit, price diambil dari DB).

Setelah order berhasil dibuat, **token meja akan dihapus** (`token = NULL`, `status = 'available'`) sehingga meja dapat di-scan kembali.

Response:
```json
{
  "success": true,
  "message": "order berhasil dibuat",
  "data": { "id": "uuid" }
}
```

#### Get Orders (outlet dari JWT)
```
GET /api/orders
Authorization: Bearer <token>
```

#### Get Order By ID
```
GET /api/orders/{id}
Authorization: Bearer <token>
```

Response:
```json
{
  "success": true,
  "message": "order fetched successfully",
  "data": {
    "id": "uuid",
    "order_name": "string",
    "order_type": "string",
    "order_status": "string",
    "order_number": "string",
    "capital_price": 0,
    "tax": 0,
    "profit": 0,
    "total": 0,
    "notes": "string | null",
    "table_id": "uuid",
    "outlet_id": "uuid",
    "created_at": "datetime",
    "updated_at": "datetime",
    "change_by": "uuid | null",
    "items": [
      {
        "id": "uuid | null",
        "order_id": "uuid | null",
        "product_id": "uuid",
        "qty": 0,
        "sub_total": 0,
        "capital_price": 0,
        "profit": 0,
        "tax": 0,
        "discount": 0,
        "notes": "string | null"
      }
    ]
  }
}
```

#### Update Order
```
PATCH /api/orders/{id}
Authorization: Bearer <token>

{
  "order_name": "string | null",
  "order_type": "string | null",
  "table_id": "uuid | null",
  "notes": "string | null",
  "update_items": [
    { "id": "uuid (order_item_id)", "qty": 2 }
  ] | null,
  "add_items": [
    { "product_id": "uuid", "qty": 1, "notes": "string | null" }
  ] | null,
  "remove_item_ids": ["uuid (order_item_id)", ...] | null
}
```

`capital_price`, `tax`, `profit`, `total` akan dihitung ulang otomatis setelah perubahan item.

#### Update Order Status
```
PATCH /api/orders/{id}/status
Authorization: Bearer <token>

{
  "status": "string"
}
```

#### Delete Order
```
DELETE /api/orders/{id}
Authorization: Bearer <token>
```

---

## Ringkasan Semua Endpoint

### Public (tanpa token)
| Method | Path |
|--------|------|
| GET | `/health` |
| POST | `/register` |
| POST | `/login` |
| GET | `/auth/google/callback?code=...` |
| GET | `/api/invites/validate/{token}` |
| POST | `/api/invites/use` |
| GET | `/api/scan/{slug}` |
| POST | `/api/scan/select-table` |
| POST | `/api/orders/consume` |

### Protected (Bearer Token) — `outlet_id` dari JWT
| Method | Path |
|--------|------|
| POST | `/api/outlets` |
| GET | `/api/outlets/me` |
| GET | `/api/outlets/{id}` |
| PATCH | `/api/outlets/{id}` |
| DELETE | `/api/outlets/{id}` |
| POST | `/api/invites` |
| DELETE | `/api/invites/{id}` |
| POST | `/api/categories` |
| GET | `/api/categories` |
| GET | `/api/categories/{category_id}` |
| PUT | `/api/categories/{category_id}` |
| DELETE | `/api/categories/{category_id}` |
| POST | `/api/products` |
| GET | `/api/products` |
| GET | `/api/products/{product_id}` |
| PATCH | `/api/products/{product_id}` |
| PATCH | `/api/products/{product_id}/available` |
| DELETE | `/api/products/{product_id}` |
| POST | `/api/qrcodes` |
| POST | `/api/qrcodes/auto` |
| GET | `/api/qrcodes` |
| GET | `/api/qrcodes/{qr_id}/tables` |
| PATCH | `/api/qrcodes/{qr_id}/regenerate` |
| POST | `/api/tables` |
| GET | `/api/tables` |
| GET | `/api/tables/{id}` |
| PATCH | `/api/tables/{id}` |
| DELETE | `/api/tables/{id}` |
| DELETE | `/api/tables/{id}/token` |
| GET | `/api/orders` |
| GET | `/api/orders/{id}` |
| PATCH | `/api/orders/{id}` |
| PATCH | `/api/orders/{id}/status` |
| DELETE | `/api/orders/{id}` |
