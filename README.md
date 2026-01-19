<h1 align="center">The Movies</h1>

<p align="center">
  <img alt="GitHub Workflow Status" src="https://img.shields.io/github/actions/workflow/status/1995parham-learning/the-movies/ci.yaml?logo=github&style=for-the-badge">
</p>

A simple REST API for managing movies, built with Rust and Axum.

## Tech Stack

- **Rust** (Edition 2024)
- **Axum** - Web framework
- **Tokio** - Async runtime
- **Serde** - Serialization/deserialization

## API Endpoints

| Method | Endpoint      | Description       |
| ------ | ------------- | ----------------- |
| GET    | `/movie`      | List all movies   |
| POST   | `/movie`      | Create a movie    |
| GET    | `/movie/{id}` | Get a movie by ID |
| PUT    | `/movie/{id}` | Update a movie    |
| DELETE | `/movie/{id}` | Delete a movie    |

### Create a Movie

```http
POST /movie
Content-Type: application/json

{
  "id": "1",
  "name": "The Shawshank Redemption",
  "year": 1994,
  "was_good": true
}
```

**Response:** `201 Created` with created movie

### List All Movies

```http
GET /movie
```

**Response:** `200 OK` with array of movies

### Get a Movie

```http
GET /movie/{id}
```

**Response:** `200 OK` with movie, or `404 Not Found`

### Update a Movie

```http
PUT /movie/{id}
Content-Type: application/json

{
  "id": "1",
  "name": "Updated Name",
  "year": 1994,
  "was_good": true
}
```

**Response:** `200 OK` with updated movie, or `404 Not Found`

### Delete a Movie

```http
DELETE /movie/{id}
```

**Response:** `204 No Content`, or `404 Not Found`

## Running

```bash
cargo run
```

The server starts at `http://127.0.0.1:3000`.

## Development

```bash
# Check code
cargo check

# Run linter
cargo clippy

# Format code
cargo fmt
```
