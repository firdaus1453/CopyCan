<h1 align="center">
  📋 CopyCan
</h1>

<p align="center">
  <b>Never lose a copied text again.</b><br/>
  A blazing-fast, ultra-lightweight clipboard history manager for macOS — written entirely in Rust.
</p>

<p align="center">
  <a href="https://github.com/firdaus1453/CopyCan/releases/latest"><img src="https://img.shields.io/github/v/release/firdaus1453/CopyCan?style=flat-square&color=blue" alt="Release" /></a>
  <img src="https://img.shields.io/badge/platform-macOS-lightgrey?style=flat-square&logo=apple" />
  <img src="https://img.shields.io/badge/language-Rust-orange?style=flat-square&logo=rust" />
  <img src="https://img.shields.io/badge/license-MIT-green?style=flat-square" />
  <img src="https://img.shields.io/github/actions/workflow/status/firdaus1453/CopyCan/release.yml?style=flat-square&label=build" />
</p>

<p align="center">
  <a href="#-quick-install">Install</a> •
  <a href="#-features">Features</a> •
  <a href="#-how-it-works">How It Works</a> •
  <a href="#-build-from-source">Build</a> •
  <a href="#-contributing">Contributing</a>
</p>

---

<p align="center">
  <img src="assets/screenshot.png" width="520" alt="CopyCan in action" />
</p>

---

## 🚨 The Problem

You're working. You copy something important. You get distracted, copy something else — **and your previous clipboard is gone forever.**

macOS only keeps **one** item in the clipboard. There's no undo. There's no history. Just gone.

**CopyCan fixes this.** It silently watches your clipboard in the background and remembers everything you copy — so you never have to worry about losing a copied text ever again.

## ✨ Features

| | Feature | Description |
|---|---|---|
| ⚡️ | **Instant Access** | Press **`⌘ Shift V`** to pop up your full clipboard history at the cursor |
| 🪶 | **Featherweight** | ~880 KB binary. Pure Rust, zero Electron, zero web views |
| 👻 | **Invisible** | Runs as a menu bar icon only — no Dock icon, no windows |
| 💾 | **Persistent** | Saves your last 50 clips to disk. Survives reboots |
| 🔒 | **Private** | 100% offline. No telemetry. No cloud. Your data stays on your Mac |
| 🍎 | **Universal** | Runs natively on both Intel and Apple Silicon Macs |

## 📥 Quick Install

1. Download **`CopyCan.dmg`** from the [latest release](https://github.com/firdaus1453/CopyCan/releases/latest)
2. Open the `.dmg` and drag `CopyCan.app` → `Applications`
3. Launch from Spotlight or Launchpad
4. **First launch:** Right-click → Open → Open (to bypass Gatekeeper)

> **Tip:** Grant **Accessibility** permission in *System Settings → Privacy & Security → Accessibility* for the `⌘ Shift V` shortcut to work.

## 🧠 How It Works

```
You copy something → CopyCan saves it → Press ⌘⇧V → Pick any past clip → Cmd+V to paste
```

CopyCan polls the system clipboard every 500ms. When it detects new text, it saves it to a local JSON file (`~/.clipboard_history.json`). Your history is always one shortcut away.

**Menu bar preview:**
```
━━ Clipboard History ━━
  Meeting notes from standup...
  https://github.com/firdaus1453/CopyCan
  SELECT * FROM users WHERE...
  +62 812-XXXX-XXXX
───────────────────────
✕ Quit
```

Click any item → it's copied back to your clipboard, ready to paste.

## 🔨 Build From Source

**Prerequisites:** [Rust](https://rustup.rs/)

```bash
git clone https://github.com/firdaus1453/CopyCan.git
cd CopyCan
cargo build --release
./target/release/CopyCan
```

**Package as `.app`:**
```bash
chmod +x create_app_bundle.sh
./create_app_bundle.sh
# → Creates CopyCan.app (drag to /Applications)
```

**Package as `.dmg`:**
```bash
chmod +x create_dmg.sh
./create_dmg.sh
```

Or just push a git tag (`v*`) to trigger automated GitHub Actions release with Universal Binary.

## 🏗 Project Structure

```
CopyCan/
├── src/main.rs                # All application logic (single-file architecture)
├── create_app_bundle.sh       # .app packaging script
├── create_dmg.sh              # .dmg packaging script
├── assets/                    # Icon (.icns) and screenshots
└── .github/workflows/         # CI/CD: auto-build Universal Binary + DMG on tag push
```

## ⚙️ Tech Stack

| Crate | Role |
|---|---|
| `tray-icon` + `muda` | Native macOS menu bar integration |
| `arboard` | Clipboard read/write |
| `global-hotkey` | System-wide `⌘⇧V` shortcut |
| `tao` | Headless event loop (no Dock icon) |
| `serde` + `serde_json` | Persistent history storage |

No async runtime. No heavy frameworks. Just Rust and native macOS APIs.

## 📊 Performance

| Metric | Value |
|---|---|
| Binary size | ~880 KB |
| Refresh interval | 500 ms polling |
| Memory usage | < 5 MB |

## 🤝 Contributing

CopyCan is open source under the [MIT License](LICENSE). Contributions are welcome!

- 🐛 Found a bug? [Open an issue](https://github.com/firdaus1453/CopyCan/issues)
- 💡 Have an idea? [Start a discussion](https://github.com/firdaus1453/CopyCan/discussions)
- 🔧 Want to contribute? Fork, branch, PR — the usual!

---

<p align="center">
  <sub>Built with 🩵 by <a href="https://github.com/firdaus1453">@firdaus1453</a></sub>
</p>
