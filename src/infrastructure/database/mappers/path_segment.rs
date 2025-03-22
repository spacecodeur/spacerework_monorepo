use crate::{
    domain::{entities::path_segment::PathSegment, types::enums::SegmentTypeEnum},
    infrastructure::database::entities::{
        path_segment, path_segment_type, sea_orm_active_enums::SegmentTypeName,
    },
};

impl From<SegmentTypeName> for SegmentTypeEnum {
    fn from(value: SegmentTypeName) -> Self {
        match value {
            SegmentTypeName::Directory => SegmentTypeEnum::Directory,
            SegmentTypeName::Lesson => SegmentTypeEnum::Lesson,
        }
    }
}

pub fn map_path_segment(
    path_segment_from_db: path_segment::Model,
    path_segment_type_from_db: path_segment_type::Model,
) -> PathSegment {
    PathSegment {
        name: path_segment_from_db.name,
        segment_parent: None,
        trainer: None,
        segment_type: path_segment_type_from_db.name.into(),
        children: Vec::new(),
    }
}
