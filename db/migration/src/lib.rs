pub use sea_orm_migration::prelude::*;

mod m20241221_185614_create_user_table;
mod m20241221_190742_create_accounts_table;
mod m20241221_191426_create_transactions_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241221_185614_create_user_table::Migration),
            Box::new(m20241221_190742_create_accounts_table::Migration),
            Box::new(m20241221_191426_create_transactions_table::Migration),
        ]
    }
}
