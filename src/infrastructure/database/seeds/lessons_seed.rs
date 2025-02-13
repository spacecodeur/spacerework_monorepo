use sea_orm_migration::sea_orm::{entity::*, query::*};

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        cake::ActiveModel {
            name: Set("Cheesecake".to_owned()),
            ..Default::default()
        }
        .insert(db)
        .await?;

        Ok(())
    }
}

pub async fn lessons_seed(db: &DatabaseConnection) -> Result<(), sea_orm::DbErr> {
    let lesson1 = ActiveModel {
        id: Set(1),
        content: Set("Introduction to Rust".to_string()),
        trainer_id: Set(1),
    };

    let lesson2 = ActiveModel {
        id: Set(2),
        content: Set("Advanced Rust".to_string()),
        trainer_id: Set(2),
    };

    Lesson::insert_many(vec![lesson1, lesson2])
        .exec(db)
        .await?;

    println!("✅ Lessons seeded !");
    Ok(())
}
