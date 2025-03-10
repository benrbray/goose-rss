use std::{error::Error, path::Path};

use diesel::{prelude::*, sqlite::Sqlite};

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

////////////////////////////////////////////////////////////////////////////////

pub fn open_connection(database_path: &Path) -> SqliteConnection {
  let database_path = database_path.join("goose.db");
  let database_path_str = database_path.to_str().expect("error building db path");
  
  // TODO: gracefully handle database connection error
  SqliteConnection::establish(database_path_str)
  .expect(format!("error connecting to database: {}", database_path_str).as_str())
}

/* ---- migrations ---------------------------------------------------------- */

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn run_migrations(connection: &mut impl MigrationHarness<Sqlite>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    // See the documentation for `MigrationHarness` for all available methods.
    connection.run_pending_migrations(MIGRATIONS)?;
    Ok(())
}