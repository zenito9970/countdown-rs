Terminal Countdown Timer (Rust implementation)
===

The original implementation of this was inspired by Anton Medvedev.  
Original implementation => [antonmedv/countdown](https://github.com/antonmedv/countdown)

Usage
---

Specify duration in go format `1h2m3s` .

```
countdown-rs 25s
```

Add command with `&&` to run after countdown.

```
countdown-rs 1m30s && say "Hello, world"
```

Press `Esc` or `Ctrl+C` to stop countdown without running next command.

Install
---

```
git clone https://github.com/zenito9970/countdown-rs.git
cd countdown-rs
cargo install --path .
```

License
---

MIT
