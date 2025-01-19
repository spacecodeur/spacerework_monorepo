use tuono_lib::{Props, Request, Response};
use diesel::prelude::*;
use tuono_app::{establish_connection, models::*};

fn main() -> Lesson {
    use tuono_app::schema::lesson::dsl::*;
    
    let connection = &mut establish_connection();
    let results= lesson
    .select(Lesson::as_select())
    .load(connection)
    .expect("Error loading lesson");
    
    return Lesson{
        id: results[0].id,
        content: results[0].content.clone()
    };
}
    
#[tuono_lib::handler]
async fn get_server_side_props(_req: Request) -> Response {
    Response::Props(Props::new(main()))
}