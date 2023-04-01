use crate::core::data_type::DataType;

pub struct SQLiteProperty {
    pub name: String,
    pub data_type: DataType,
    // for embedded objects
    pub collection_index: Option<u16>,
}

impl SQLiteProperty {
    pub fn new(name: &str, data_type: DataType, collection_index: Option<u16>) -> Self {
        SQLiteProperty {
            name: name.to_string(),
            data_type,
            collection_index,
        }
    }
}

pub struct SQLiteCollection {
    pub name: String,
    pub properties: Vec<SQLiteProperty>,
}

impl SQLiteCollection {
    pub fn new(name: String, properties: Vec<SQLiteProperty>) -> Self {
        Self { name, properties }
    }
}
