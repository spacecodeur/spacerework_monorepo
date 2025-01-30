use tuono_lib::axum::response::IntoResponse;
use tuono_lib::Request;
use tuono_lib::axum::http::StatusCode;

#[tuono_lib::api(GET)]
pub async fn save(_req: Request) -> impl IntoResponse {
    (StatusCode::OK, "coucou")
}