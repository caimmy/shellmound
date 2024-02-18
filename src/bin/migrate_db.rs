// src/main.rs

use sea_orm_migration::prelude::*;
// src/main.rs

use futures::executor::block_on;
use sea_orm::{Database, DbErr};
use shellmound::migrator::Migrator;

const DATABASE_URL: &str = "mysql://shellmound:abcd1234@127.0.0.1:3306/shellmound";

async fn run() -> Result<(), DbErr> {
    let db = Database::connect(DATABASE_URL).await?;
    let schema_manager = SchemaManager::new(&db); // To investigate the schema

    Migrator::refresh(&db).await?;
    assert!(schema_manager.has_table("bakery").await?);

    Ok(())
}

fn main() {
    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }
}
