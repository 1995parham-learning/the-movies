use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use axum::{
    extract::{Path, State, Json as PJson}, http::StatusCode, response::{IntoResponse, Json}, routing::{get, post}, Router
};

use serde::{Serialize, Deserialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
struct Movie {
    id: String,
    name: String,
    year: u16,
    was_good: bool
}

#[derive(Clone)]
struct AppState {
    data: Arc<Mutex<HashMap<String, Movie>>>,
}


#[tokio::main]
async fn main() {
    let data: HashMap<String, Movie> = HashMap::new();
    let state = AppState {
        data: Arc::new(Mutex::new(data)),
    };

    let app = Router::new()
        .route("/movie", post(create_movie))
        .route("/movie/{id}", get(get_movie))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_movie(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    match state.data.lock().expect("mutex was poisoned").get(&id) {
        Some(movie) => return (StatusCode::OK, Json(json!(movie))),
        None => return (StatusCode::NOT_FOUND, Json(json!("movie not found")))
    }
}

async fn create_movie(
    State(state): State<AppState>,
    PJson(payload): PJson<Movie>,
) -> StatusCode {
    let mut s = state.data.lock().expect("mutex was poisoned");

    s.insert(payload.id.clone(), payload);

    StatusCode::CREATED
}
