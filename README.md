# rust-playground

## Day 1 - Install rust on Mac

Inspired by this article: http://zsiciarz.github.io/24daysofrust/index.html

Initially I had `rust` on `brew`, but it appeared not the best option due to:
- No std library source code package is attached. You'll need to download it by yourself.
- `rustup` is not available to mange multipe rust versions.

Hence I've uninstalled and download `rustup` from https://rustup.rs/
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

All rust components are installed under `~/.cargo`. Ensure you have `~/.cargo/bin` in PATH.

For IDE, I chose IntelliJ + Rust plugin, because I use Jetbrain's products for all other languages.

Started reading The Rust Programming Language.

Completed Chapter 1 and 2 by writing guessing_game.

## Day 2 - Further practice

Continue Chapter 3.

Learned variables, if expression, loop/while/for

