use sea_orm::{DatabaseConnection, EntityTrait, Set};
use crate::entities::trainer::{ActiveModel, Entity as Trainer};

pub async fn trainers_seed(db: &DatabaseConnection) -> Result<(), sea_orm::DbErr> {
    let trainer1 = ActiveModel {
        id: Set(1),
        pseudo: Set("Trainer_A".to_string()),
    };

    let trainer2 = ActiveModel {
        id: Set(2),
        pseudo: Set("Trainer_B".to_string()),
    };

    Trainer::insert_many(vec![trainer1, trainer2])
        .exec(db)
        .await?;

    println!("✅ Trainers seeded !");
    Ok(())
}
