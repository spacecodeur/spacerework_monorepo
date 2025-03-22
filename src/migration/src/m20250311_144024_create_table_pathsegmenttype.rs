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
                    .as_enum(PathSegmentTypeName::Enum)
                    .values([PathSegmentTypeName::Directory, PathSegmentTypeName::Lesson])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(PathSegmentType::Table)
                    .col(pk_auto(PathSegmentType::Id))
                    .col(
                        ColumnDef::new(PathSegmentType::Name)
                            .enumeration(
                                PathSegmentTypeName::Enum,
                                [PathSegmentTypeName::Directory, PathSegmentTypeName::Lesson],
                            )
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PathSegmentType::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(PathSegmentTypeName::Enum).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum PathSegmentType {
    Table,
    Id,
    Name,
}

#[derive(DeriveIden)]
enum PathSegmentTypeName {
    #[sea_orm(iden = "segment_type_name")]
    Enum,
    #[sea_orm(iden = "directory")]
    Directory,
    #[sea_orm(iden = "lesson")]
    Lesson,
}
