use std::error::Error;

use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use crate::db::schema;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

#[derive(Insertable, Queryable, Selectable)]
#[diesel(table_name = schema::files)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct File {
    pub filepath: String,
    pub hash: String,
}

pub fn get_file(pool: &mut Pool<ConnectionManager<SqliteConnection>>, path: &str) -> Option<File> {
    use schema::files::dsl::*;
    files
        .find(path)
        .select(File::as_select())
        .first(&mut pool.get().unwrap())
        .optional()
        .expect("query for file")
}

pub fn insert_file(pool: &mut Pool<ConnectionManager<SqliteConnection>>, file: File) {
    use schema::files::dsl::*;
    diesel::insert_into(files)
        .values(&file)
        .execute(&mut pool.get().unwrap())
        .expect("save new file");
}

pub fn run_migrations(
    conn: &mut impl MigrationHarness<diesel::sqlite::Sqlite>,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    conn.run_pending_migrations(MIGRATIONS)?;
    Ok(())
}
