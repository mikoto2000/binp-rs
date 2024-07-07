use std::collections::HashMap;

use serde::Deserialize;

use crate::types::Endianness;

#[derive(Copy, Clone, Deserialize, Debug)]
pub enum DataType {
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

// コンフィグの要素は、以下 2 種類のどれかとなる
#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum ConfigItem {
    FLAGS(BitFlagConfigItem),
    UINT8(BasicConfigItem),
    UINT16(BasicConfigItem),
    UINT32(BasicConfigItem),
    UINT64(BasicConfigItem),
    INT8(BasicConfigItem),
    INT16(BasicConfigItem),
    INT32(BasicConfigItem),
    INT64(BasicConfigItem),
}

// 数値データの単位を表す構造体
#[derive(Deserialize, Clone, Debug)]
pub struct BasicConfigItem {
    // 表示名
    pub name: String,
    // ファイル先頭からのオフセット
    pub offset: usize,
    // オフセットから何バイト読み込むか
    pub size: usize,
    // データタイプ
    #[serde(alias = "type")]
    pub data_type: DataType,
    // エンディアン
    pub endianness: Option<Endianness>,
    // 特定の値を別の文字にマッピングする定義
    pub value_label: Option<HashMap<String, String>>,
}

// ビットフラグデータの単位を表す構造体
#[derive(Deserialize, Debug)]
pub struct BitFlagConfigItem {
    // 表示名
    pub name: String,
    // ファイル先頭からのオフセット
    pub offset: usize,
    // データタイプ
    #[serde(alias = "type")]
    pub data_type: DataType,
    // type が FLAGS の時のみ利用されるフィールド
    pub layout: Vec<LayoutItem>,
}

// ビットフラグの 1 ビットを表す構造体
#[derive(Deserialize, Debug)]
pub struct LayoutItem {
    // 表示名
    pub name: String,
    // ビットフラグのビット位置
    pub position: usize,
    // ビットが 1 だった時に表示する値
    pub true_label: Option<String>,
    // ビットが 0 だった時に表示する値
    pub false_label: Option<String>,
}
