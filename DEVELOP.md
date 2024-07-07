# DEVELOP

## Release

### Pre requirements:

```sh
sudo apt update
sudo apt install -y \
    build-essential \
    gcc-aarch64-linux-gnu \
    gcc-mingw-w64-x86-64
sudo apt install -y \
    gcc-multilib
cargo install cross
```

## x86_64 Linux build:

```sh
cross build --release --target x86_64-unknown-linux-musl
```

## aarch64 Linux build:

```sh
# なぜか cargo でエラーを出した後でないと cross でも失敗する
cargo build --release --target aarch64-unknown-linux-musl
cross build --release --target aarch64-unknown-linux-musl
```

## x86_64 Windows build:

```sh
cross build --release --target x86_64-pc-windows-gnu
```

