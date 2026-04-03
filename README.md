# Data Privacy Vault (Rust)

A small HTTP service that acts as a Data Privacy Vault for the CodingChallenges.fyi “Build Your Own Data Privacy Vault” challenge.

The service exposes a HTTP API that:

- Accepts sensitive data and returns **tokens** (`/tokenize`).
- Accepts tokens and returns the original data if found (`/detokenize`).

Internally, values are stored encrypted using AES‑GCM, either in memory or in Redis.

---

## Features

- **Step 1 – In‑memory tokenization service**
  - `POST /tokenize`: create tokens for sensitive fields.
  - `POST /detokenize`: resolve tokens back to the original values.
  - Data can be stored in an in‑memory map (default for quick testing).

- **Step 2 – Persistent encrypted storage**
  - Optional Redis backend for persistent storage.
  - All values are stored encrypted using AES‑256‑GCM (AEAD).

---

## API

### Tokenize

**Endpoint**

```http
POST /tokenize
```

**Request**

```json
{
  "id": "req-12345",
  "data": {
    "field1": "value1",
    "field2": "value2",
    "fieldn": "valuen"
  }
}
```

**Success response**

- Status: `201 Created`

```json
{
  "id": "req-12345",
  "data": {
    "field1": "t6yh4f6",
    "field2": "gh67ned",
    "fieldn": "bnj7ytb"
  }
}
```


### Detokenize

**Endpoint**

```http
POST /detokenize
```

**Request**

```json
{
  "id": "req-33445",
  "data": {
    "field1": "t6yh4f6",
    "field2": "gh67ned",
    "field3": "invalid token"
  }
}
```

**Success response**

```json
{
  "field1": {
    "found": true,
    "value": "value1"
  },
  "field2": {
    "found": true,
    "value": "value2"
  },
  "field3": {
    "found": false,
    "value": ""
  }
}
```

**Error responses**

- `400 Bad Request` – malformed JSON, `data` is not an object, non‑string tokens.
- `500 Internal Server Error` – storage or crypto errors.

---

## Storage

The vault can use either in‑memory storage or Redis.

- **In‑memory**: good for Step 1 and quick tests.
- **Redis**: good for Step 2 and persistence, storing `token -> ciphertext` mappings.

```sh
docker run --name vault-redis -p 6379:6379 -d redis:7
```