use std::time::Duration;

use diesel::{
    connection::SimpleConnection,
    r2d2::{ConnectionManager, CustomizeConnection, Pool},
    SqliteConnection,
};

#[derive(Debug)]
pub struct ConnectionOptions {
    pub enable_wal: bool,
    pub busy_timeout: Option<Duration>,
}

impl CustomizeConnection<diesel::sqlite::SqliteConnection, diesel::r2d2::Error>
    for ConnectionOptions
{
    fn on_acquire(
        &self,
        conn: &mut diesel::sqlite::SqliteConnection,
    ) -> Result<(), diesel::r2d2::Error> {
        (|| {
            if self.enable_wal {
                conn.batch_execute("PRAGMA journal_mode = WAL; PRAGMA synchronous = NORMAL;")?;
            }

            if let Some(timeout) = self.busy_timeout {
                conn.batch_execute(&format!("PRAGMA busy_timeout = {}", timeout.as_millis()))?;
            }
            Ok(())
        })()
        .map_err(diesel::r2d2::Error::QueryError)
    }
}

pub fn get_connection_pool() -> Pool<ConnectionManager<SqliteConnection>> {
    Pool::builder()
        .max_size(16)
        .connection_customizer(Box::new(ConnectionOptions {
            enable_wal: true,
            busy_timeout: Some(Duration::from_secs(5)),
        }))
        .build(ConnectionManager::<SqliteConnection>::new(
            crate::config::get().database_location.as_str(),
        ))
        .expect("pool build")
}
