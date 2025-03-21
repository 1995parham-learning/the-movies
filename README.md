# The Movies

## Introduction

Create Axum server with the following endpoints:

1. GET `/movie/{id}` - This should return a movie given the ID
2. POST `/movie` - this should save move in a DB (`HashMap<String, Movie>`). This movie will be sent via a JSON payload.

As a bonus: implement a caching layer, so we don't need to make expensive "DB" lookups, etc.
