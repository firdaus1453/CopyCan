<h1 align="center">
  📋 CopyCan
</h1>

<p align="center">
  <strong>A lightweight, blazing fast, and native macOS clipboard history manager built with Rust.</strong>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/macOS-10.15%2B-lightgrey?style=for-the-badge&logo=apple" />
  <img src="https://img.shields.io/badge/Language-Rust-orange?style=for-the-badge&logo=rust" />
  <img src="https://img.shields.io/badge/License-MIT-blue?style=for-the-badge" />
</p>

---

## What is CopyCan?

**CopyCan** is an extremely lightweight, completely background-running clipboard history manager designed strictly for macOS. It respects your screen real-estate by completely hiding its dock icon and only staying accessible from the top macOS Menu Bar.

It automatically remembers the string texts you've copied, saving you from the horrible tragedy of accidentally replacing your clipboard just before pasting! 

### Features ✨
* 🪶 **Hyper-Lightweight**: Consumes < 10MB of memory thanks to Rust.
* 🚀 **Popup Shortcut**: Press **`Cmd + Shift + V`** to display your history exactly at your cursor!
* 💾 **Persistence Save**: Remembers up to 50 of your recent clips, effortlessly recovering them even after a laptop reboot (`~/.clipboard_history.json`).
* 👻 **Dock Hidden**: It behaves like a pure system accessory—no annoying active app bounds in your dock.

---

## 🛠️ Installation

### Quick Install (.dmg)
1. Go to the [Releases Setup](#) (Will be automatically filled by Github Actions)
2. Download the `CopyCan.dmg` file.
3. Open the `.dmg`, and drag the `CopyCan` app to your `Applications/` folder.
4. Launch it from Launchpad or Spotlight.
5. *(Optional)* macOS might prompt you about downloading an unsigned app. Just go to **System Settings > Privacy & Security** and click **Open Anyway**. You may also need to grant it **Accessibility / Input Monitoring** permissions for the `Cmd + Shift + V` shortcut to work effectively!

### Build From Source
If you have Rust installed, just clone and build the release:
```bash
git clone https://github.com/your-username/CopyCan.git
cd CopyCan
cargo build --release
```

---

## 🚀 How To Use

1. Run the app contextually. A `📋` icon will appear in your top right Menu Bar.
2. Copy some text: `Cmd + C` (as normal).
3. Need an older clip? Either click the menu icon or press **`Cmd + Shift + V`**.
4. Click the desired text in the dropdown, and then execute **`Cmd + V`** to paste it seamlessly!

## Contribute
As CopyCan is open-sourced under the MIT license, you are welcome to fix bugs, add new native interactions, or bundle new cross-platform targets!

*(Built with 🩵)*
