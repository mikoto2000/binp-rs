use std::{fs::File, io::BufReader};

use clap::Parser;
use config::ConfigItem;

mod config;
mod option_parser;

fn main() {
    let options = option_parser::Options::parse();

    // 1. コンフィグを読み込む
    // yaml ファイルを読み込み、 Reader 化
    let config_path = options.config;
    let config_file = File::open(config_path).unwrap();
    let config_reader = BufReader::new(config_file);
    let _config: Vec<ConfigItem> = serde_yaml::from_reader(config_reader).unwrap();

    // 2. ファイルを読み込みバイナリの配列に変換

    // 3. コンフィグを走査しながらバイナリをパース

    // 4. 「3.」の結果を表示

    println!("Hello, world!");
}
