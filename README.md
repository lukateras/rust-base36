# `rust-base36`

Base36 encoder/decoder for Rust. See: <https://en.wikipedia.org/wiki/Base36>

Base36 is the most compact regular encoding that is valid in domain names.

## Usage

Add the crate to your project's `Cargo.toml`:

```toml
base36 = "1"
```

You are all set:

```rust
fn main() {
    let buf = [0, 1, 2, 63, 64, 65, 127, 128, 129];
    println!("{}", base36::encode(&buf));
}
```

```
$ cargo run
jvqhhz7lkzl
```
