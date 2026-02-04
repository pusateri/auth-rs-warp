use r2d2;
use r2d2_sqlite::SqliteConnectionManager;
// auto-connect to DB, keep pool global
lazy_static::lazy_static! {
    pub static ref DB_CONN_POOL: Pool = connect_DB();
}

pub type Pool = r2d2::Pool<SqliteConnectionManager>;
pub type PooledConnection = r2d2::PooledConnection<SqliteConnectionManager>;

// METHODS
pub fn get() -> Result<PooledConnection, r2d2::Error> {
    DB_CONN_POOL.get()
}

fn connect_DB() -> Pool {
    let manager = SqliteConnectionManager::file("file.db");
    return r2d2::Pool::new(manager).expect("Failed to create pool.");
}
