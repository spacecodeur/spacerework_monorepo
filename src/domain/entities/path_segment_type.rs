use crate::domain::types::enums::SegmentTypeName;

pub struct PathSegmentType {
    pub id: i32,
    pub name: Option<SegmentTypeName>,
}
