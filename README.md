A toy game created to play around with the [ggez](https://github.com/ggez/ggez) framework for Rust. Involves Ferris the Crustacean shooting up the enemies of Rust: inefficiencies, undefined behaviour, and so on.

![Demo](./demo.gif)

It should be possible to execute it with a simple `cargo run`.

To improve compilation times, you might want to use the [mold linker](https://github.com/rui314/mold) by placing this in `.cargo/config.toml`:

```toml
[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=/usr/bin/mold"]
```
