use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use axum::{
    Router,
    extract::{Json as EJson, Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::get,
};

use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Movie {
    id: String,
    name: String,
    year: u16,
    was_good: bool,
}

#[derive(Clone)]
struct AppState {
    data: Arc<RwLock<HashMap<String, Movie>>>,
}

fn app() -> Router {
    let data: HashMap<String, Movie> = HashMap::new();
    let state = AppState {
        data: Arc::new(RwLock::new(data)),
    };

    Router::new()
        .route("/movie", get(list_movies).post(create_movie))
        .route("/movie/{id}", get(get_movie).put(update_movie).delete(delete_movie))
        .with_state(state)
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app()).await.unwrap();
}

async fn list_movies(State(state): State<AppState>) -> impl IntoResponse {
    let movies: Vec<Movie> = state
        .data
        .read()
        .expect("lock was poisoned")
        .values()
        .cloned()
        .collect();

    Json(movies)
}

async fn get_movie(Path(id): Path<String>, State(state): State<AppState>) -> impl IntoResponse {
    match state.data.read().expect("lock was poisoned").get(&id) {
        Some(movie) => (StatusCode::OK, Json(json!(movie))),
        None => (StatusCode::NOT_FOUND, Json(json!("movie not found"))),
    }
}

async fn update_movie(
    Path(id): Path<String>,
    State(state): State<AppState>,
    EJson(payload): EJson<Movie>,
) -> impl IntoResponse {
    let mut s = state.data.write().expect("lock was poisoned");

    if !s.contains_key(&id) {
        return (StatusCode::NOT_FOUND, Json(json!("movie not found")));
    }

    let movie = Movie { id, ..payload };
    s.insert(movie.id.clone(), movie.clone());

    (StatusCode::OK, Json(json!(movie)))
}

async fn delete_movie(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let mut s = state.data.write().expect("lock was poisoned");

    match s.remove(&id) {
        Some(_) => StatusCode::NO_CONTENT,
        None => StatusCode::NOT_FOUND,
    }
}

async fn create_movie(
    State(state): State<AppState>,
    EJson(payload): EJson<Movie>,
) -> impl IntoResponse {
    let mut s = state.data.write().expect("lock was poisoned");

    s.insert(payload.id.clone(), payload.clone());

    (StatusCode::CREATED, Json(payload))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, header},
    };
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    #[tokio::test]
    async fn create_movie_returns_created() {
        let response = app()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/movie")
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        r#"{"id":"1","name":"Test Movie","year":2024,"was_good":true}"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let movie: Movie = serde_json::from_slice(&body).unwrap();
        assert_eq!(movie.id, "1");
        assert_eq!(movie.name, "Test Movie");
        assert_eq!(movie.year, 2024);
        assert!(movie.was_good);
    }

    #[tokio::test]
    async fn get_movie_not_found() {
        let response = app()
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/movie/999")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: String = serde_json::from_slice(&body).unwrap();
        assert_eq!(body, "movie not found");
    }

    #[tokio::test]
    async fn create_and_get_movie() {
        let app = app();

        // Create a movie
        let create_response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/movie")
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        r#"{"id":"1","name":"The Matrix","year":1999,"was_good":true}"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(create_response.status(), StatusCode::CREATED);

        // Get the movie
        let get_response = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/movie/1")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(get_response.status(), StatusCode::OK);

        let body = get_response.into_body().collect().await.unwrap().to_bytes();
        let movie: Movie = serde_json::from_slice(&body).unwrap();
        assert_eq!(movie.id, "1");
        assert_eq!(movie.name, "The Matrix");
        assert_eq!(movie.year, 1999);
        assert!(movie.was_good);
    }

    #[tokio::test]
    async fn list_movies_empty() {
        let response = app()
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/movie")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let movies: Vec<Movie> = serde_json::from_slice(&body).unwrap();
        assert!(movies.is_empty());
    }

    #[tokio::test]
    async fn list_movies_with_data() {
        let app = app();

        // Create a movie first
        app.clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/movie")
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        r#"{"id":"1","name":"Test Movie","year":2024,"was_good":true}"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        // List movies
        let response = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/movie")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let movies: Vec<Movie> = serde_json::from_slice(&body).unwrap();
        assert_eq!(movies.len(), 1);
    }

    #[tokio::test]
    async fn update_movie_success() {
        let app = app();

        // Create a movie
        app.clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/movie")
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        r#"{"id":"1","name":"Old Name","year":2020,"was_good":false}"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        // Update the movie
        let response = app
            .oneshot(
                Request::builder()
                    .method("PUT")
                    .uri("/movie/1")
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        r#"{"id":"1","name":"New Name","year":2024,"was_good":true}"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let movie: Movie = serde_json::from_slice(&body).unwrap();
        assert_eq!(movie.name, "New Name");
        assert_eq!(movie.year, 2024);
        assert!(movie.was_good);
    }

    #[tokio::test]
    async fn update_movie_not_found() {
        let response = app()
            .oneshot(
                Request::builder()
                    .method("PUT")
                    .uri("/movie/999")
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        r#"{"id":"999","name":"Test","year":2024,"was_good":true}"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn delete_movie_success() {
        let app = app();

        // Create a movie
        app.clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/movie")
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        r#"{"id":"1","name":"Test","year":2024,"was_good":true}"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        // Delete the movie
        let response = app
            .oneshot(
                Request::builder()
                    .method("DELETE")
                    .uri("/movie/1")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NO_CONTENT);
    }

    #[tokio::test]
    async fn delete_movie_not_found() {
        let response = app()
            .oneshot(
                Request::builder()
                    .method("DELETE")
                    .uri("/movie/999")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
