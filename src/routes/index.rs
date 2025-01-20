use md_to_html::greet;

use tuono_lib::{Props, Request, Response};

#[tuono_lib::handler]
async fn get_server_side_props(_req: Request) -> Response {
    Response::Props(Props::new(greet("Marco")))
}

