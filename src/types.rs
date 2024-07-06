use serde::Deserialize;

use crate::config::DataType;

#[derive(Deserialize, Copy, Clone, Debug)]
pub enum Endianness {
    BIG,
    LITTLE,
    NONE
}

#[derive(Deserialize, Debug)]
pub enum Type {
    UINT8,
    UINT16,
    UINT32,
    UINT64,
    INT8,
    INT16,
    INT32,
    INT64,
    FLAGS,
}

#[derive(Debug)]
pub struct OutputItem {
    pub endianness: Endianness,
    pub name: String,
    pub offset: usize,
    pub size: usize,
    pub data_type: DataType,
    pub value: String,
}
