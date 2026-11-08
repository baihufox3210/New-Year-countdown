# New Year Countdown

A minimal web page built with **Rust + WebAssembly** that shows the current time, a countdown to the next New Year, and this year's progress.

**Live demo:** https://baihufox3210.github.io/New-Year-countdown/

## Development

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk --locked
trunk serve
```

Pushing to `main` builds and deploys the site to GitHub Pages via GitHub Actions.
