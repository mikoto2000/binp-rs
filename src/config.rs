use serde::{Deserialize, Serialize};

// コンフィグの要素は、以下 5 種類のどれかとなる
// - UINT8
// - UINT16
// - UINT32
// - UINT64
// - FLAGS
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ConfigItem {
    UINT8(BasicConfigItem),
    UINT16(BasicConfigItem),
    UINT32(BasicConfigItem),
    UINT64(BasicConfigItem),
    FLAGS(BitFlagConfigItem),
}

// 数値データの単位を表す構造体
#[derive(Serialize, Deserialize, Debug)]
pub struct BasicConfigItem {
    // 表示名
    name: String,
    // ファイル先頭からのオフセット
    offset: u8,
    // オフセットから何バイト読み込むか
    size: u8,
    // エンディアン
    endianness: Option<String>,
}

// ビットフラグデータの単位を表す構造体
#[derive(Serialize, Deserialize, Debug)]
pub struct BitFlagConfigItem {
    // 表示名
    name: String,
    // ファイル先頭からのオフセット
    offset: u8,
    // オフセットから何バイト読み込むか
    size: u8,
    // エンディアン
    endianness: Option<String>,
    // type が FLAGS の時のみ利用されるフィールド
    layout: Vec<LayoutItem>,
}

// ビットフラグの 1 ビットを表す構造体
#[derive(Serialize, Deserialize, Debug)]
pub struct LayoutItem {
    // 表示名
    name: String,
    // ビットフラグのビット位置
    position: u8,
    // ビットが 1 だった時に表示する値
    true_label: Option<String>,
    // ビットが 0 だった時に表示する値
    false_label: Option<String>,
}
