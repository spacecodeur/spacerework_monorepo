use sea_orm_migration::{
    prelude::{extension::postgres::Type, *},
    schema::*,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(SegmentTypeName::Enum)
                    .values([SegmentTypeName::Directory, SegmentTypeName::Lesson])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(SegmentType::Table)
                    .col(pk_auto(SegmentType::Id))
                    .col(ColumnDef::new(SegmentType::Name).enumeration(
                        SegmentTypeName::Enum,
                        [SegmentTypeName::Directory, SegmentTypeName::Lesson],
                    ))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_type(Type::drop().name(SegmentTypeName::Enum).to_owned())
            .await?;

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

#[derive(DeriveIden)]
enum SegmentTypeName {
    #[sea_orm(iden = "Name")]
    Enum,
    #[sea_orm(iden = "directory")]
    Directory,
    #[sea_orm(iden = "lesson")]
    Lesson,
}
