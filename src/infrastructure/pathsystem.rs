use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use super::database::entities::path_segment;

pub async fn get_pathsegment_from_path(
    db_connection: &DatabaseConnection,
    path: &str,
    trainer_id: i32,
) -> Result<path_segment::Model, &'static str> {
    let parts: Vec<&str> = path.split('/').collect();

    let mut segment = path_segment::Model {
        id: 0,
        name: String::new(),
        segment_parent_id: None,
        trainer_id: None,
        segment_type_id: 0,
    };

    for (i, part) in parts.iter().enumerate() {
        let mut query =
            path_segment::Entity::find().filter(path_segment::Column::Name.contains(*part));

        if i == 0 {
            query = query.filter(path_segment::Column::TrainerId.eq(trainer_id));
        } else {
            query = query.filter(path_segment::Column::SegmentParentId.eq(segment.id));
        }
        segment = match query.one(db_connection).await.unwrap() {
            Some(segment) => segment,
            None => return Err("segment not found !"),
        };
    }
    Ok(segment)
}
