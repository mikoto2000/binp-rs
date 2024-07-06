use std::{
    fs::File,
    io::{BufReader, Read},
};

use clap::Parser;
use config::ConfigItem;
use tabled::{settings::{object::Columns, style::HorizontalLine, Disable, Style}, Table};
use types::OutputItem;

mod binary_parser;
mod config;
mod option_parser;
mod types;

fn main() {
    let options = option_parser::Options::parse();

    // 1. コンフィグを読み込む
    // yaml ファイルを読み込み、 Reader 化
    let config_path = options.config;
    let config_file = File::open(config_path).unwrap();
    let config_reader = BufReader::new(config_file);
    let config: Vec<ConfigItem> = serde_yaml::from_reader(config_reader).unwrap();

    // 2. ファイルを読み込みバイナリの配列に変換
    let mut file = File::open(options.arg).expect("バイナリファイルのオープンに失敗しました。");
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)
        .expect("バイナリファイルの読み込みに失敗しました。");

    // 3. コンフィグを走査しながらバイナリをパース
    let results: Vec<OutputItem> = config
        .iter()
        .flat_map(|item| binary_parser::parse(&buf, item))
        .collect();

    // 4. 「3.」の結果を表示
    let mut table = Table::new(results);
    table.with(
        Style::ascii()
            .horizontals([(1, HorizontalLine::inherit(Style::ascii()))])
            .remove_horizontal(),
    );

    if !options.all {
        table.with(Disable::column(Columns::single(4)));
        table.with(Disable::column(Columns::single(3)));
        table.with(Disable::column(Columns::single(2)));
        table.with(Disable::column(Columns::single(0)));
    }

    println!("{}", table);
}
