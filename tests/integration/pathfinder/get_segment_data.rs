use app::domain::entities::{path_segment, user};
use app::infrastructure::pathsystem::get_segment_data;
use sea_orm::{ActiveModelTrait, EntityTrait};

use sea_orm::Set;
use tuono_lib::tokio;

use crate::integration::common;
use crate::integration::common::database::{
    cleanup, get_app_db_name, get_db_connection_uri, get_temp_db_name,
};

#[test]
fn is_invalid_persistent_segment() {
    // check if asked path is not stored in db
}

#[test]
fn get_content_lesson_from_valid_segment() {}

#[tokio::test]
async fn get_childs_of_valid_segment() {
    let db_connection_uri = get_db_connection_uri();
    let app_db_name = get_app_db_name();
    let temp_db_name = get_temp_db_name();

    let test_db = common::database::setup(&db_connection_uri, &app_db_name, &temp_db_name).await;

    let trainer = user::ActiveModel {
        id: Set(1),
        pseudo: Set("john".to_owned()),
    };

    trainer
        .insert(&test_db)
        .await
        .expect("could not insert trainer");

    let segments = vec![
        (1, "tata", None, Some(1)),
        (2, "tata2", None, Some(1)),
        (3, "tata3", None, Some(1)),
        (4, "toto", Some(1), Some(1)),
        (5, "tata", Some(1), None),
        (6, "coursjs.md", Some(2), None),
        (7, "coursnode.md", Some(3), None),
        (8, "coursnode2.md", Some(3), None),
        (9, "coursphp.md", Some(5), None),
        (10, "tata", Some(5), None),
    ];

    let models: Vec<path_segment::ActiveModel> = segments
        .into_iter()
        .map(
            |(id, name, segment_parent_id, trainer_id)| path_segment::ActiveModel {
                id: Set(id),
                name: Set(name.to_owned()),
                segment_parent_id: Set(segment_parent_id),
                trainer_id: Set(trainer_id),
            },
        )
        .collect();

    path_segment::Entity::insert_many(models)
        .exec(&test_db)
        .await
        .expect("could not insert path_segment");

    let result = get_segment_data("tata/tata", 1);
    assert_eq!(result.is_err(), false);

    let childs = result.unwrap();

    assert_eq!(childs[0], "coursphp.md");
    assert_eq!(childs[1], "tata");

    cleanup(&db_connection_uri, &app_db_name, &temp_db_name).await;
}

#[test]
fn get_empty_array_of_empty_root_trainer() {}
