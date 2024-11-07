pub struct PostgresIndexer {
    pool: sqlx::AnyPool,
}

impl PostgresIndexer {
    pub fn new(pool: sqlx::AnyPool) -> Result<Self, sqlx::Error> {
        Ok(Self { pool })
    }
}

