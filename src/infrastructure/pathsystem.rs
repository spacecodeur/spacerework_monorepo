use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use super::super::domain::entities::path_segment;
use super::database::pool::DatabaseService;

pub async fn get_segment_data(
    path: &str,
    trainer_id: i32,
    DB_CONNECTION: &DatabaseConnection,
) -> Result<Vec<&'static str>, &'static str> {
    let db: DatabaseService = DatabaseService::init().await;

    let parts: Vec<&str> = path.split('/').collect();

    for (i, part) in parts.iter().enumerate() {
        let mut query =
            path_segment::Entity::find().filter(path_segment::Column::Name.contains(*part));
        if i == 0 {
            query = query.filter(path_segment::Column::TrainerId.eq(trainer_id));
        }
        let result: Option<path_segment::Model> = query.one(DB_CONNECTION).await.unwrap();

        println!("le resultat est : {:?}", result)
        // let   trainers: user::Entity::find().all(&*db.connection).await.unwrap();
    }

    let _ = path;
    let _ = trainer_id;
    Ok(vec!["coursphp.md", "tata"])
}

// pub fn is_dir_db_check => retourne id dir ou lesson

// et ensuite on check les results du dessus : en fonction de, on recherche dans la db les info du dir ou de la lesson
