use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    version,
    about = "バイナリファイルからデータを抽出するツール。",
)]
pub struct Options {
    #[arg(short, long, help = "設定ファイルパス")]
    config: String,
    #[arg(short, long, help = "ファイル更新時に再表示します")]
    watch: bool,
    #[arg(
        short,
        long,
        help = "指定したポーリング間隔(ミリ秒)で再表示します",
        default_value = "1000"
    )]
    polling: u32,
    #[arg(
        short,
        long,
        help = "name, value 以外のすべての項目(endianness, offset, size, type)を表示する"
    )]
    all: bool,

    #[arg(
        help = "パース対象のバイナリファイル"
    )]
    arg: String,
}
