use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SegmentType::Table)
                    .col(pk_auto(SegmentType::Id))
                    .col(string(SegmentType::Name))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SegmentType::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum SegmentType {
    Table,
    Id,
    Name,
}
