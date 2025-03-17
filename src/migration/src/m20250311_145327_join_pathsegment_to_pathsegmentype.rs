use sea_orm_migration::{prelude::*, schema::*};

use crate::m20250311_144024_create_table_pathsegmenttype::PathSegmentType;

#[derive(DeriveIden)]
enum PathSegment {
    Table,
    SegmentTypeId,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(PathSegment::Table)
                    .add_column(integer(PathSegment::SegmentTypeId))
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("FK_segment_type")
                            .from_tbl(PathSegment::Table)
                            .from_col(PathSegment::SegmentTypeId)
                            .to_tbl(PathSegmentType::Table)
                            .to_col(PathSegmentType::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(PathSegment::Table)
                    .drop_foreign_key(Alias::new("FK_segment_type"))
                    .drop_column(PathSegment::SegmentTypeId)
                    .to_owned(),
            )
            .await
    }
}
