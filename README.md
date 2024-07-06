# Binp rs - Binary Parser

バイナリファイルからデータを抽出するツールの Rust 実装。

`offset`, `size` `type`, `endianness` を指定して、バイナリファイルのどこからどこまでをどのように解釈するかを指定できる。

([mikoto2000/binp: バイナリファイルからデータを抽出するツール。](https://github.com/mikoto2000/binp) の再実装)


# 使い方

```sh
Usage: binp-rs [OPTIONS] --config <CONFIG> <ARG>

Arguments:
  <ARG>  パース対象のバイナリファイル

Options:
  -c, --config <CONFIG>    設定ファイルパス
  -w, --watch              ファイル更新時に再表示します
  -p, --polling <POLLING>  指定したポーリング間隔(ミリ秒)で再表示します [default: 1000]
  -a, --all                name, value 以外のすべての項目(endianness, offset, size, type)を表示する
  -h, --help               Print help
  -V, --version            Print version
```

# インストール方法

gem コマンドでインストールしてください。

```sh
gem install binp
```

# 設定例

以下内容のバイナリファイルをパースする場合について説明する。

■ `example.bin`

```
+----+----+----+----+----+----+----+----+----+----+----+----+----+----+-------+
|  0 |  1 |  2 |  3 |  4 |  5 |  6 |  7 |  8 |  9 | 10 | 11 | 12 | 13 |    14 |
+----+----+----+----+----+----+----+----+----+----+----+----+----+----+-------+
| 00 | 00 | 00 | 00 | 00 | 00 | 00 | 01 | 00 | 00 | 00 | 01 | 00 | 01 |    01 |
+---------------------------------------+-------------------+---------+-------+
| UINT64                                | UINT32            |  UINT16 | UINT8 |
+---------------------------------------+-------------------+---------+-------+
※ エンディアンはリトルエンディアン
```

以下のように、設定ファイルに `name`, `offset`, `size`, `type` を記述する。

■ `setting.yaml`

```yaml
- name: UINT64_value
  offset: 0
  size: 8
  type: UINT64
  endianness: LITTLE
- name: UINT32_value
  offset: 8
  size: 4
  type: UINT32
  endianness: LITTLE
- name: UINT16_value
  offset: 12
  size: 2
  type: UINT16
  endianness: LITTLE
- name: UINT8_value
  offset: 14
  size: 1
  type: UINT8
  endianness: LITTLE
```

`binary_parser.rb` に設定ファイルとバイナリファイルを指定して実行する。

実行結果は以下のようになる。

```sh
$ ruby binary_parser.rb -c setting.yaml example.bin
  +--------------+-------------------+
  | name         | value             |
  +--------------+-------------------+
  | UINT64_value | 72057594037927936 |
  | UINT32_value | 16777216          |
  | UINT16_value | 256               |
  | UINT8_value  | 1                 |
  +--------------+-------------------+
```

`-a` オプションで追加の情報が出力される。

```sh
$ ruby binary_parser.rb -a -c setting.yaml example.bin
  +------------+--------------+--------+------+--------+-------------------+
  | endianness | name         | offset | size | type   | value             |
  +------------+--------------+--------+------+--------+-------------------+
  | LITTLE     | UINT64_value | 0      | 8    | UINT64 | 72057594037927936 |
  | LITTLE     | UINT32_value | 8      | 4    | UINT32 | 16777216          |
  | LITTLE     | UINT16_value | 12     | 2    | UINT16 | 256               |
  | LITTLE     | UINT8_value  | 14     | 1    | UINT8  | 1                 |
  +------------+--------------+--------+------+--------+-------------------+
```

# TODO:

- [x] : テーブル形式で表示
- [ ] : 文字列型サポート
    - [ ] : UTF8
- [x] : ビットフラグサポート
- [ ] : type からの size 自動設定
- [ ] : json 形式で表示
