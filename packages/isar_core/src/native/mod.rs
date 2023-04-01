mod index;
mod mdbx;
pub mod native_collection;
pub mod native_filter;
pub mod native_insert;
pub mod native_instance;
pub mod native_object;
pub mod native_query;
pub mod native_query_builder;
pub mod native_reader;
pub mod native_txn;
pub mod native_writer;
mod schema_manager;

pub(crate) const NULL_BYTE: u8 = 0;
pub(crate) const NULL_BOOL: u8 = 0;
pub(crate) const FALSE_BOOL: u8 = 1;
pub(crate) const TRUE_BOOL: u8 = 2;
pub(crate) const NULL_INT: i32 = i32::MIN;
pub(crate) const NULL_LONG: i64 = i64::MIN;
pub(crate) const NULL_FLOAT: f32 = f32::NAN;
pub(crate) const NULL_DOUBLE: f64 = f64::NAN;
pub(crate) const MAX_OBJ_SIZE: u32 = 2 << 24;
