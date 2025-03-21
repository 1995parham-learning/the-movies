# The Movies

## Introduction

Create Axum server with the following endpoints:

1. GET `/movie/{id}` - This should return a movie given the ID
2. POST `/movie` - this should save move in a DB (`HashMap<String, Movie>`). This movie will be sent via a JSON payload.

As a bonus: implement a caching layer, so we don't need to make expensive "DB" lookups, etc.

## How to run?

After running the server using `cargo run`, you can create movie as follows:

```bash
curl 127.0.0.1:3000/movie -X POST -d '{ "id": "1", "name": "good movie", "year": 2025, "was_good": true }' -H 'Content-Type: application/json'
```

then request them using:

```bash
curl 127.0.0.1:3000/movie/1
```
