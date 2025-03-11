use app::domain::entities::{path_segment, user};
use app::infrastructure::pathsystem::get_pathsegment_from_path;
use sea_orm::{ActiveModelTrait, EntityTrait};
use space_macros::use_temp_db;

use sea_orm::Set;
use tuono_lib::tokio;

use crate::integration::common;
use crate::integration::common::database::{
    get_app_db_name, get_db_connection_uri, get_temp_db_name,
};

#[test]
fn is_invalid_persistent_segment() {}

#[test]
fn get_content_lesson_from_valid_segment() {}

#[use_temp_db]
async fn get_childs_of_valid_segment() {
    let trainer = user::ActiveModel {
        id: Set(1),
        pseudo: Set("john".to_owned()),
    };

    trainer
        .insert(&db_connection)
        .await
        .expect("could not insert trainer");

    #[rustfmt::skip]
    let segments = vec![
        path_segment::Model {id: 1, name: "dir1".to_string(),       segment_parent_id: None,    trainer_id: Some(1)},
        path_segment::Model {id: 2, name: "dir2".to_string(),       segment_parent_id: Some(1), trainer_id: None},
        path_segment::Model {id: 3, name: "dir3".to_string(),       segment_parent_id: Some(2), trainer_id: None},
        path_segment::Model {id: 4, name: "lesson.md".to_string(),  segment_parent_id: Some(2), trainer_id: None},
        path_segment::Model {id: 5, name: "dir4".to_string(),       segment_parent_id: Some(3), trainer_id: None},
    ];

    // Convert Vec<Model> into Vec<ActiveModel>
    let active_segments: Vec<path_segment::ActiveModel> =
        segments.into_iter().map(Into::into).collect();

    path_segment::Entity::insert_many(active_segments)
        .exec(&db_connection)
        .await
        .expect("could not insert path_segment");

    let mut result = get_pathsegment_from_path(&db_connection, "dir1/dir2/dir3/dir4", 1).await;
    assert_eq!(result.is_err(), false);
    assert_eq!(result.unwrap().name, "dir4");

    result = get_pathsegment_from_path(&db_connection, "dir1/dir2/lesson.md", 1).await;
    assert_eq!(result.is_err(), false);
    assert_eq!(result.unwrap().name, "lesson.md");

    result = get_pathsegment_from_path(&db_connection, "dir1/this_dir_does_not_exist", 1).await;
    assert_eq!(result.is_err(), true);
    assert_eq!(result.unwrap_err(), "segment not found !");
}

#[test]
fn get_empty_array_of_empty_root_trainer() {}
