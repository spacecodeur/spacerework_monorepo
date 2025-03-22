use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, SelectTwo};

use crate::domain::entities::path_segment::PathSegment;

use super::database::{
    entities::{path_segment, path_segment_type, sea_orm_active_enums::SegmentTypeName},
    mappers::path_segment::map_path_segment,
};

pub async fn get_pathsegment_from_path(
    db_connection: &DatabaseConnection,
    path: &str,
    trainer_id: i32,
) -> Result<PathSegment, String> {
    let parts: Vec<&str> = path.split('/').collect();

    let mut query: SelectTwo<path_segment::Entity, path_segment_type::Entity>;
    let mut path_segment: Option<path_segment::Model> = None;
    let mut path_segment_type: Option<path_segment_type::Model> = None;

    for (i, part) in parts.iter().enumerate() {
        query = path_segment::Entity::find()
            .find_also_related(path_segment_type::Entity)
            .filter(path_segment::Column::Name.contains(*part));

        if i == 0 {
            query = query.filter(path_segment::Column::TrainerId.eq(trainer_id));
        } else {
            match path_segment_type.unwrap().name {
                SegmentTypeName::Directory => {
                    query = query
                        .filter(path_segment::Column::SegmentParentId.eq(path_segment.unwrap().id))
                }
                _ => {
                    return Err(format!(
                        "In path '{}', '{}' (id : '{}') is not a directory",
                        path,
                        part,
                        path_segment.unwrap().id
                    ))
                }
            };
        }

        match query.one(db_connection).await {
            Ok(Some((segment, segment_type))) => {
                path_segment = Some(segment);
                path_segment_type = match segment_type {
                    Some(path_segment_type) => Some(path_segment_type),
                    None => return Err("segment_type not found !".to_string()),
                };
            }
            Ok(None) => return Err("segment not found !".to_string()),
            Err(err) => {
                return Err(format!(
                    "Erreur lors de la récupération des données : {:?}",
                    err
                ))
            }
        }
    }

    match (path_segment, path_segment_type) {
        (Some(segment), Some(segment_type)) => Ok(map_path_segment(segment, segment_type)),
        _ => Err(format!(
            "An error occurred while parsing the path '{}'",
            path
        )),
    }
}
