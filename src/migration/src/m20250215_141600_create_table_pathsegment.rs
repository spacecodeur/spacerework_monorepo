use sea_orm_migration::{prelude::*, schema::*};

use crate::m20250213_230915_create_table_user::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PathSegment::Table)
                    .col(pk_auto(PathSegment::Id))
                    .col(string_len(PathSegment::Name, 50))
                    .col(integer_null(PathSegment::SegmentParentId))
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .from_tbl(PathSegment::Table)
                            .from_col(PathSegment::SegmentParentId)
                            .to_tbl(PathSegment::Table)
                            .to_col(PathSegment::Id),
                    )
                    .col(integer_null(PathSegment::TrainerId))
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .from_tbl(PathSegment::Table)
                            .from_col(PathSegment::TrainerId)
                            .to_tbl(User::Table)
                            .to_col(User::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PathSegment::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum PathSegment {
    Table,
    Id,
    Name,
    SegmentParentId,
    TrainerId,
}
