use r2d2;
use diesel::mysql::MysqlConnection;
use r2d2_diesel::ConnectionManager;
use settings::Database;

type ManagedMysqlConn = ConnectionManager<MysqlConnection>;
type Pool = r2d2::Pool<ManagedMysqlConn>;

pub struct DbConn {
    pub master: r2d2::PooledConnection<ManagedMysqlConn>,
}

pub struct DbCollection {
    pub db_conn_pool: Pool,
}

pub fn init_db(database_config: &Database) -> DbCollection {
    let db_manager =
        ConnectionManager::<MysqlConnection>::new(database_config.database_url.clone());
    let db_pool =
        r2d2::Pool::new(db_manager).expect("Failed to create db pool.");

    DbCollection {
        db_conn_pool: db_pool,
    }
}
