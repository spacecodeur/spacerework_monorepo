use app::domain::entities::lesson::Model as Lesson;
use md_to_html::md_to_html;
use serde::Serialize;
use tuono_lib::{Props, Request, Response};

#[derive(Serialize)]
struct Path {
    status: String,
    value: String,
}

#[derive(Serialize)]
struct Result {
    path: Path,
    lesson: Option<Lesson>,
    lesson_html: Option<String>,
}

fn path(path: &str) -> Path {
    let mut tmp = "path".to_string();
    if path.ends_with(".md") {
        tmp = "lesson".to_string()
    } else if path.ends_with(".md/edit") {
        tmp = "edit".to_string()
    }

    let resultat: String = path
        .split('/') // Divise la chaîne par les "/"
        .skip(3) // Ignore les trois premiers éléments (le premier est vide à cause du '/')
        .collect::<Vec<_>>() // Collecte le reste dans un vecteur
        .join("/");

    Path {
        status: tmp,
        value: resultat,
    }
}

fn get_lesson() -> Option<Lesson> {
    return Some(Lesson {
        id: 3,
        content: "# Hello !\n(from src/components/lesson/ReadMode.tsx file)".to_owned(),
        user_id: 1,
    });
}

#[tuono_lib::handler]
async fn get_server_side_props(_req: Request) -> Response {
    let mut result = Result {
        path: path(&_req.uri.to_string()),
        lesson: get_lesson(),
        lesson_html: None,
    };

    if result.path.status == "lesson" {
        result.lesson_html = Some(md_to_html(result.lesson.as_ref().unwrap().content.as_str()));
    }

    Response::Props(Props::new(result))
}
