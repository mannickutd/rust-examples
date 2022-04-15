use super::model::{Todo};

pub mod db_context;
mod todo;

pub type Database<'c> = db_context::Database<'c>;
pub type Table<'c, T> = db_context::Table<'c, T>;
