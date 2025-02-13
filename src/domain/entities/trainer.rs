use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "trainer")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    #[sea_orm(column_type = "String(Some(50))")] // max_length 50
    pub pseudo: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::lesson::Entity")]
    Lesson,
}

impl ActiveModelBehavior for ActiveModel {}
