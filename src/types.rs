use serde::Deserialize;
use tabled::Tabled;

#[derive(Deserialize, Copy, Clone, Debug)]
pub enum Endianness {
    BIG,
    LITTLE
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

#[derive(Debug, Tabled)]
pub struct OutputItem {
    pub endianness: String,
    pub name: String,
    pub offset: usize,
    pub size: usize,
    pub data_type: String,
    pub value: String,
}
