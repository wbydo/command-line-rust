# command-line-rust

# Reference
[Rustの練習帳](https://www.oreilly.co.jp/books/9784814400584/)

# Basic Usage
```
cargo build
cargo run
cargo check
```


# [Release Build](https://doc.rust-jp.rs/book-ja/ch01-03-hello-cargo.html#%E3%83%AA%E3%83%AA%E3%83%BC%E3%82%B9%E3%81%AB%E5%90%91%E3%81%91%E3%81%9F%E3%83%93%E3%83%AB%E3%83%89)

```
cargo build --release
cargo run --release
```

# `src/bin` 内のコマンドの実行の仕方

```
cargo run --quiet --bin false
```