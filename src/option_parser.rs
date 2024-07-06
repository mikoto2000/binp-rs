use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about = "バイナリファイルからデータを抽出するツール。")]
pub struct Options {
    #[arg(short, long, help = "設定ファイルパス")]
    pub config: String,
    #[arg(short, long, help = "ファイル更新時に再表示します")]
    pub watch: bool,
    #[arg(
        short,
        long,
        help = "指定したポーリング間隔(ミリ秒)で再表示します",
        default_value = "1000"
    )]
    pub polling: Option<u64>,
    #[arg(
        short,
        long,
        help = "name, value 以外のすべての項目(endianness, offset, size, type)を表示する"
    )]
    pub all: bool,

    #[arg(help = "パース対象のバイナリファイル")]
    pub arg: String,
}
