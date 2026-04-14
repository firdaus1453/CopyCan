---

<img src="assets/screenshot.png" width="300" align="right" alt="CopyCan screenshot" style="border-radius:8px"/>

A lightweight macOS clipboard history manager written in Rust. Automatically remembers your copied text and lets you paste them seamlessly using a global hotkey.

Built for modern macOS (Intel & Apple Silicon) running macOS 10.15+.

## ✨ Features
- ⚡️ **Ultra-Lightweight & Fast** — Written in 100% pure Rust. No Electron, no web views, no heavy frameworks.
- 🎯 **Popup Shortcut** — Press **`Cmd + Shift + V`** to instantly display your clipboard history exactly at your cursor!
- 🥶 **Zero Bloat** — Extremely low memory footprint (< 10 MB RAM) running silently in the background.
- 🖥 **Native macOS Experience** — Integrates seamlessly into the Menu Bar and respects your screen real-estate with absolutely **zero Dock Icon**.
- 🔒 **Privacy First** — Fully local and offline processing. Your clipboard data never leaves your machine.
- 💾 **Safe Recovery** — Persists your last 50 clips locally (`~/.clipboard_history.json`), surviving reboots so you never lose your data.

## 📸 Screenshot
<p align="center">
  <img src="assets/screenshot.png" width="600" alt="CopyCan screenshot" style="border-radius: 8px; box-shadow: 0 4px 14px rgba(0,0,0,0.1);" />
</p>

## Menu Bar Display
The application sits quietly in your menu bar. Click to see the detailed dropdown or press `Cmd + Shift + V`.

```text
━━ Clipboard History ━━
  [1] Copied text snippet...
  [2] Another important text...
  [3] https://github.com/firdaus1453...
───────────────────────
✕ Quit
```
*Clicking any item immediately copies it to your active clipboard, ready for `Cmd + V`.*

## 📥 Installation (For Regular Users)
The easiest way to install CopyCan is to download the pre-built `.dmg` file.

1. Go to the [Releases page](https://github.com/firdaus1453/CopyCan/releases/latest) and download `CopyCan.dmg`.
2. Double-click the downloaded `.dmg` file to open it.
3. **Drag and drop** the `CopyCan.app` icon into the **Applications** folder shortcut.
4. Open your Applications folder (or Launchpad).
5. **Important:** Because this is an indie open-source app, macOS Gatekeeper might block it the first time. To open it safely:
   - **Right-click** (or Control-click) on `CopyCan.app`.
   - Select **Open** from the menu.
   - Click **Open** again on the pop-up warning.
6. The app will quietly start and appear in your menu bar at the top of the screen!
*Note: Make sure to give CopyCan **Accessibility Permissions** in System Settings to allow the `Cmd + Shift + V` hotkey to function properly.*

---

### Prerequisites
Install Rust (if not already installed):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

### Build & Run
```bash
# Clone or navigate to the project
git clone https://github.com/firdaus1453/CopyCan.git
cd CopyCan

# Build release binary
cargo build --release

# Run the application
./target/release/CopyCan
```

The app will appear in your macOS menu bar — no window opens.

### Package as .app Bundle
```bash
chmod +x create_app_bundle.sh
./create_app_bundle.sh
```

This creates `CopyCan.app` which you can:
- Move to `/Applications/`
- Double-click to launch
- Add to Login Items via System Settings

The `.app` bundle has `LSUIElement=true` set, meaning it runs as a menu bar-only app with no Dock icon.

### Package as .dmg for Distribution
```bash
chmod +x create_dmg.sh
./create_dmg.sh
```

Alternatively, just push a tag `v*` to trigger the GitHub Actions workflow which automatically builds a **Universal Binary** and publishes a `.dmg` release!

## Project Structure
```
CopyCan/
├── Cargo.toml                 # Dependencies and build config
├── create_app_bundle.sh       # .app bundle packaging script
├── create_dmg.sh              # DMG packaging script
├── README.md                  # This file
├── src/
│   └── main.rs                # Entry point, event loop, clipboard polling, and tray UI
├── assets/                    # App icon and screenshots
└── .github/workflows/         # GitHub Actions CI/CD pipeline
```

## Dependencies
| Crate | Purpose |
|---|---|
| `tray-icon` & `muda` | macOS NSMenu and StatusItem integration |
| `arboard` | Cross-platform clipboard reading/writing |
| `global-hotkey` | Native global keyboard accelerators |
| `tao` | Headless event loop (`ActivationPolicy::Accessory`) |
| `serde` & `serde_json` | JSON persistence configuration |

No async runtime, no heavy frameworks.

## Performance
| Metric | Value |
|---|---|
| Refresh interval | 500 ms polling |
| Memory usage | < 10 MB |

## License
MIT
