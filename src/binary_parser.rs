use byteorder::{BigEndian, ByteOrder, LittleEndian};

use crate::{
    config::{self, BasicConfigItem, BitFlagConfigItem, ConfigItem},
    types::{Endianness, OutputItem},
};

pub fn parse(binary: &Vec<u8>, config_item: &ConfigItem) -> Vec<OutputItem> {
    match config_item {
        ConfigItem::FLAGS(e) => {
            parse_bitflag_value(binary, e)
        }
        ConfigItem::UINT8(e)
        | ConfigItem::UINT16(e)
        | ConfigItem::UINT32(e)
        | ConfigItem::UINT64(e)
        | ConfigItem::INT8(e)
        | ConfigItem::INT16(e)
        | ConfigItem::INT32(e)
        | ConfigItem::INT64(e) => vec![parse_number_value(binary, e)],
    }
}

fn parse_number_value(binary: &Vec<u8>, config_item: &BasicConfigItem) -> OutputItem {
    match &config_item.data_type {
        config::DataType::UINT8 => {
            create_output_item(config_item, (binary[config_item.offset] as u8).to_string())
        }
        config::DataType::UINT16 | config::DataType::UINT32 | config::DataType::UINT64 => {
            match config_item.endianness {
                Some(Endianness::BIG) => match config_item.size {
                    2 => create_output_item(
                        config_item,
                        BigEndian::read_u16(
                            &binary[config_item.offset..config_item.offset + config_item.size],
                        )
                        .to_string(),
                    ),
                    4 => create_output_item(
                        config_item,
                        BigEndian::read_u32(
                            &binary[config_item.offset..config_item.offset + config_item.size],
                        )
                        .to_string(),
                    ),
                    8 => create_output_item(
                        config_item,
                        BigEndian::read_u64(
                            &binary[config_item.offset..config_item.offset + config_item.size],
                        )
                        .to_string(),
                    ),
                    _ => panic!("no support size!"),
                },
                Some(Endianness::LITTLE) => match config_item.size {
                    2 => create_output_item(
                        config_item,
                        LittleEndian::read_u16(
                            &binary[config_item.offset..config_item.offset + config_item.size],
                        )
                        .to_string(),
                    ),
                    4 => create_output_item(
                        config_item,
                        LittleEndian::read_u32(
                            &binary[config_item.offset..config_item.offset + config_item.size],
                        )
                        .to_string(),
                    ),
                    8 => create_output_item(
                        config_item,
                        LittleEndian::read_u64(
                            &binary[config_item.offset..config_item.offset + config_item.size],
                        )
                        .to_string(),
                    ),
                    _ => panic!("no support size!"),
                },
                None => panic!("Unknown endianness!"),
            }
        }
        config::DataType::INT8 => {
            create_output_item(config_item, (binary[config_item.offset] as i8).to_string())
        }
        config::DataType::INT16 | config::DataType::INT32 | config::DataType::INT64 => {
            match config_item.endianness {
                Some(Endianness::BIG) => match config_item.size {
                    2 => create_output_item(
                        config_item,
                        BigEndian::read_i16(
                            &binary[config_item.offset..config_item.offset + config_item.size],
                        )
                        .to_string(),
                    ),
                    4 => create_output_item(
                        config_item,
                        BigEndian::read_i32(
                            &binary[config_item.offset..config_item.offset + config_item.size],
                        )
                        .to_string(),
                    ),
                    8 => create_output_item(
                        config_item,
                        BigEndian::read_i64(
                            &binary[config_item.offset..config_item.offset + config_item.size],
                        )
                        .to_string(),
                    ),
                    _ => panic!("no support size!"),
                },
                Some(Endianness::LITTLE) => match config_item.size {
                    2 => create_output_item(
                        config_item,
                        LittleEndian::read_i16(
                            &binary[config_item.offset..config_item.offset + config_item.size],
                        )
                        .to_string(),
                    ),
                    4 => create_output_item(
                        config_item,
                        LittleEndian::read_i32(
                            &binary[config_item.offset..config_item.offset + config_item.size],
                        )
                        .to_string(),
                    ),
                    8 => create_output_item(
                        config_item,
                        LittleEndian::read_i64(
                            &binary[config_item.offset..config_item.offset + config_item.size],
                        )
                        .to_string(),
                    ),
                    _ => panic!("no support size!"),
                },
                None => panic!("Unknown endianness!"),
            }
        }
        _ => panic!("not supported!"),
    }
}

fn create_output_item(config_item: &BasicConfigItem, value: String) -> OutputItem {
    OutputItem {
        endianness: format!("{:?}", config_item.endianness.unwrap()),
        name: config_item.name.clone(),
        offset: config_item.offset,
        size: config_item.size,
        data_type: format!("{:?}", config_item.data_type),
        value: value,
    }
}
fn parse_bitflag_value(binary: &Vec<u8>, config_item: &BitFlagConfigItem) -> Vec<OutputItem> {
    config_item
        .layout
        .iter()
        .map(|l| {
            // 1. 指定されたビットが立っているかを判定する
            let position = l.position;
            let target_byte = binary[config_item.offset];
            let is_true = (target_byte & (1 << position)) != 0;

            // 2. 結果に応じてラベルを導出
            let value = if is_true {
                l.true_label.clone().unwrap_or("1".to_string())
            } else {
                l.false_label.clone().unwrap_or("0".to_string())
            };

            OutputItem {
                endianness: "-".to_string(),
                name: l.name.clone(),
                offset: config_item.offset,
                size: 1,
                data_type: format!("{:?}", config_item.data_type),
                value: value,
            }
        })
        .collect()
}
