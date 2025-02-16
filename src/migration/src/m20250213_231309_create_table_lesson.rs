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
                    .table(Lesson::Table)
                    .if_not_exists()
                    .col(pk_auto(Lesson::Id))
                    .col(string(Lesson::Content))
                    .col(integer(Lesson::UserId))
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .from_tbl(Lesson::Table)
                            .from_col(Lesson::UserId)
                            .to_tbl(User::Table)
                            .to_col(User::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Lesson::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Lesson {
    Table,
    Id,
    UserId,
    Content,
}
