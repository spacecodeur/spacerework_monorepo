use app::{domain::entities::user, infrastructure::database::pool::DatabaseService};
use sea_orm::EntityTrait;
use serde::Serialize;
use tuono_lib::{Props, Request, Response};

#[derive(Serialize)]
struct Result {
    trainers: Vec<user::Model>,
}

#[tuono_lib::handler]
async fn get_server_side_props(_req: Request) -> Response {
    let db = DatabaseService::init().await;

    Response::Props(Props::new(Result {
        trainers: user::Entity::find().all(&*db.connection).await.unwrap(),
    }))
}
