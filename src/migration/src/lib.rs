pub use sea_orm_migration::prelude::*;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250213_230915_create_table_user::Migration),
            Box::new(m20250213_231309_create_table_lesson::Migration),
            Box::new(m20250215_125449_insert_main_trainers::Migration),
            Box::new(m20250215_141600_create_table_pathsegment::Migration),
            // Box::new(m20250215_141606_insert_some_data_for_pathlesson_tests::Migration),
        ]
    }
}
mod m20250213_230915_create_table_user;
mod m20250213_231309_create_table_lesson;
mod m20250215_125449_insert_main_trainers;
mod m20250215_141600_create_table_pathsegment;
// mod m20250215_141606_insert_some_data_for_pathlesson_tests;
