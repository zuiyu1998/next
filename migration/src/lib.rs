pub use sea_orm_migration::prelude::*;

mod create_users;
mod level_template;
mod popularity;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(create_users::Migration),
            Box::new(level_template::Migration),
            Box::new(popularity::Migration),
        ]
    }
}
