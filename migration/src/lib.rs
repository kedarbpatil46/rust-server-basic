pub use sea_orm_migration::prelude::*;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260123_075929_init_trading_schema::Migration),
            Box::new(m20260127_080117_added_password_to_user_table::Migration),
        ]
    }
}

mod m20260123_075929_init_trading_schema;
mod m20260127_080117_added_password_to_user_table;
