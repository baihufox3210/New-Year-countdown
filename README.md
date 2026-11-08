# 新年倒數計時器 (New Year Countdown)

以 **Rust + WebAssembly** 打造的簡約網頁，顯示目前時間、距離下一個新年的倒數，以及今年的完成進度。

## ✨ 特色功能

- **即時時鐘**：顯示本地日期、星期與時間。
- **新年倒數**：自動以「明年 1 月 1 日 00:00:00」為目標，計算剩餘天、時、分、秒。
- **年度進度**：SVG 圓環與精確到小數點後 6 位的百分比。
- **簡約設計**：純色背景、細邊框卡片，自動跟隨系統深色 / 淺色模式，並支援手機版面。

## 🚀 技術棧

- **Rust**：全部邏輯以 Rust 撰寫，透過 `wasm-bindgen` / `web-sys` 操作 DOM。
- **Trunk**：建置與開發伺服器。
- **HTML + CSS**：無任何外部框架或字型依賴。

## 📂 檔案結構

- `src/main.rs`：WebAssembly 進入點，負責 DOM 更新與每秒計時。
- `src/countdown.rs`：純計算邏輯（倒數、年度進度、格式化），含單元測試。
- `index.html`：頁面結構（Trunk 範本）。
- `style.css`：簡約樣式。
- `Trunk.toml`：Trunk 建置設定。

## 🛠️ 開發與建置

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk --locked
```

本機開發（預設 http://127.0.0.1:8080）：

```bash
trunk serve
```

執行測試：

```bash
cargo test
```

正式建置（輸出至 `dist/`）：

```bash
trunk build --release
```

推送至 `main` 後，GitHub Actions 會自動建置並部署到 GitHub Pages
（需在 repo 的 Settings → Pages 將 Source 設為 **GitHub Actions**）。

---
*祝您新的一年，萬事如意！*
