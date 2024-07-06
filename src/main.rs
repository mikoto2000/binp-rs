use std::{
    fs::File,
    io::{stdout, BufReader, Read},
    path::Path,
    sync::mpsc,
    time::Duration,
};

use clap::Parser;
use config::ConfigItem;
use crossterm::{cursor::MoveTo, execute, terminal::{Clear, ClearType}};
use notify::{Config, PollWatcher, RecursiveMode, Watcher};
use tabled::{
    settings::{object::Columns, style::HorizontalLine, Disable, Style},
    Table,
};
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
    // 3. コンフィグを走査しながらバイナリをパース
    parse_and_format(options.arg.clone(), &config, options.all);

    if options.watch {
        let file = options.arg.clone();
        let path = Path::new(&file).as_ref();
        let (tx, rx) = mpsc::channel();
        let _watcher: Box<dyn Watcher> = if options.polling.is_some() {
            let config = Config::default()
                .with_poll_interval(Duration::from_millis(options.polling.unwrap()));
            let mut poll_watcher = PollWatcher::new(tx, config).unwrap();
            poll_watcher
                .watch(path, RecursiveMode::NonRecursive)
                .unwrap();
            Box::new(poll_watcher)
        } else {
            let mut recommended_watcher = notify::recommended_watcher(move |event| {
                let _ = tx.send(event);
            })
            .unwrap();
            recommended_watcher
                .watch(path, RecursiveMode::NonRecursive)
                .unwrap();
            Box::new(recommended_watcher)
        };

        while let Ok(res) = rx.recv() {
            match res {
                Ok(_) => {
                    parse_and_format(options.arg.clone(), &config, options.all);
                }
                Err(error) => panic!("watchi error: {error:?}"),
            }
        }
    };
}

fn parse_and_format(file_path: String, config: &Vec<ConfigItem>, is_all: bool) {
    // コンソールをリセットしてコンソールのトップへカーソルを移動
    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All), MoveTo(0,0)).unwrap();

    let file = file_path.clone();
    let mut file = File::open(file).expect("バイナリファイルのオープンに失敗しました。");
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)
        .expect("バイナリファイルの読み込みに失敗しました。");

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

    if !is_all {
        table.with(Disable::column(Columns::single(4)));
        table.with(Disable::column(Columns::single(3)));
        table.with(Disable::column(Columns::single(2)));
        table.with(Disable::column(Columns::single(0)));
    }

    println!("{}", table);
}
