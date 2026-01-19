use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use axum::{
    Router,
    extract::{Json as EJson, Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post},
};

use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
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

#[tokio::main]
async fn main() {
    let data: HashMap<String, Movie> = HashMap::new();
    let state = AppState {
        data: Arc::new(RwLock::new(data)),
    };

    let app = Router::new()
        .route("/movie", post(create_movie))
        .route("/movie/{id}", get(get_movie))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_movie(Path(id): Path<String>, State(state): State<AppState>) -> impl IntoResponse {
    match state.data.read().expect("lock was poisoned").get(&id) {
        Some(movie) => (StatusCode::OK, Json(json!(movie))),
        None => (StatusCode::NOT_FOUND, Json(json!("movie not found"))),
    }
}

async fn create_movie(State(state): State<AppState>, EJson(payload): EJson<Movie>) -> StatusCode {
    let mut s = state.data.write().expect("lock was poisoned");

    s.insert(payload.id.clone(), payload);

    StatusCode::CREATED
}
