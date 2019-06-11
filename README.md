# rust-playground

## Day 1 - Install rust on Mac

Initially I had `rust` on `brew`, but it appeared not the best option due to:
- No std library source code package is attached. You'll need to download it by yourself.
- `rustup` is not available to mange multipe rust versions.

Hence I've uninstalled and download `rustup` from https://rustup.rs/
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

All rust components are installed under `~/.cargo`. Ensure you have `~/.cargo/bin` in PATH.

For IDE, I chose IntelliJ + Rust plugin, because I use Jetbrain's products for all other languages.
