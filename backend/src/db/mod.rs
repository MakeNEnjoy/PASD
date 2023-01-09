pub mod models;
pub mod actions;
pub mod schema;

pub type DbError = Box<dyn std::error::Error + Send + Sync>;