use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "lesson")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub content: String,
    pub trainer_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::trainer::Entity",
        from = "Column::TrainerId",
        to = "super::trainer::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Trainer,
}

impl ActiveModelBehavior for ActiveModel {}
