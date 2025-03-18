pub struct PathSegment {
    pub id: i32,
    pub name: String,
    pub segment_parent_id: Option<i32>,
    pub trainer_id: Option<i32>,
    pub segment_type_id: i32,
}
