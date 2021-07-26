use deadpool_postgres::Pool;

pub struct DbPool {
    pub pool: Pool,
}