use space_md_to_html::md_to_html;

use tuono_lib::{Props, Request, Response};

#[tuono_lib::handler]
async fn get_server_side_props(_req: Request) -> Response {
    Response::Props(Props::new(md_to_html("Marco")))
}
