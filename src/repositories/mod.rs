use sqlx::PgPool;

#[derive(Clone)]
pub struct Database {
    pub writer: PgPool,
    pub reader: PgPool,
}

impl Database {
    pub fn new(writer: PgPool, reader: PgPool) -> Self {
        Self {writer, reader}
    }

    pub fn reader(&self) -> &PgPool {
        &self.reader
    }

    pub fn writer(&self) -> &PgPool {
        &self.writer
    }
}

pub mod activity_repo;