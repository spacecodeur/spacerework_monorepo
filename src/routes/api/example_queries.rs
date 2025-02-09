use tuono_lib::Request;
use tuono_lib::axum::Json;

// GET 

use serde_json::{Value, json};

#[tuono_lib::api(GET)]
async fn get(_req: Request) -> Json<Value> {
    let books = vec![
        json!({"title": "The Rust Programming Language", "author": "Steve Klabnik & Carol Nichols"}),
        json!({"title": "Programming Rust", "author": "Jim Blandy & Jason Orendorff"}),
        json!({"title": "Rust in Action", "author": "Tim McNamara"}),
    ];

    Json(json!(books))
}


// POST
use tuono_lib::axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use serde_json::from_str;

#[derive(Deserialize, Debug)]
struct NewBook {
    title: String,
}

#[derive(Serialize)]
struct ApiResponse {
    message: String,
}

#[tuono_lib::api(POST)]
async fn post(_req: Request) -> impl IntoResponse {

    // Récupérer l'en-tête "body"
    let body_header = _req.headers.get("body");

    // Vérifier si l'en-tête existe et le convertir en String
    let body_str = match body_header.and_then(|value| value.to_str().ok()) {
        Some(body) => body,
        None => return Json(ApiResponse { message: "Missing 'body' header".to_string() }).into_response(),
    };

    // Désérialiser le JSON
    let book: NewBook = match from_str(body_str) {
        Ok(data) => data,
        Err(_) => return Json(ApiResponse { message: "Invalid JSON format".to_string() }).into_response(),
    };

    println!("{:?}", book);


    // Construire la réponse
    let response = ApiResponse {
        message: format!("The book '{}' has been added with success !", book.title),
    };

    Json(response).into_response()
}