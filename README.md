# Resik Desktop (Tauri)

Versi desktop Resik Internal berbasis **Tauri v2** — ringan (~5-10MB), memakai webview sistem
(WebView2 di Windows, WKWebView di macOS, WebKitGTK di Linux). Hanya membungkus
`https://internal.resikcemerlang.com` menjadi aplikasi desktop.

## Fitur

- Jendela app sendiri (tanpa tab browser), tetap login seperti di browser.
- **Buka otomatis saat login** — toggle lewat ikon tray (klik kanan ikon Resik di tray → centang
  "Buka otomatis saat login"). Disimpan oleh sistem: registry `Run` (Windows), LaunchAgent (macOS),
  autostart `.desktop` (Linux).
- **Keep running** — menutup window hanya menyembunyikan ke tray, app tetap jalan. Keluar lewat
  menu tray "Keluar".

## Struktur

```
desktop-tauri/
├── frontend/index.html     # fallback (window utama memuat URL web app)
├── src-tauri/
│   ├── src/lib.rs          # window + tray + autostart
│   ├── Cargo.toml
│   ├── tauri.conf.json     # productName, icons, bundle
│   ├── capabilities/       # izin (autostart)
│   └── icons/              # icon aplikasi
└── package.json
```

## Prasyarat

- Node.js + npm
- Rust (via https://rustup.rs): `rustup default stable`
- Windows: WebView2 Runtime (otomatis terpasang di Win10/11; di **Windows 7/8** harus install
  WebView2 Runtime secara terpisah). macOS: Xcode command line tools. Linux: WebKitGTK
  (`sudo apt install libwebkit2gtk-4.1-dev build-essential ...` sesuai distro).

## Menjalankan

```bash
npm install
npm run tauri dev      # mode pengembangan
npm run tauri build    # build release + installer
```

## Hasil Build

- Windows: `src-tauri/target/release/bundle/nsis/Resik Internal_0.1.0_x64-setup.exe` (+ `.msi`)
- macOS: perlu di-build di mesin macOS → `.dmg`
- Linux: perlu di-build di mesin Linux → `.AppImage` / `.deb`

## Catatan Windows 7 / 8

Tauri memakai WebView2. Di Win7/8 pastikan **WebView2 Runtime** sudah terinstall di tiap PC
(unduh sekali, install silent: `MicrosoftEdgeWebView2RuntimeInstallerX64.exe /silent /install`).