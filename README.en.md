# mdpeek

An ultra-lightweight single-file Markdown reader for Windows.

<p align="center"><a href="README.md">中文</a> | English</p>

- **Single exe**: ~9 MB, no runtime dependencies
- **Tiny footprint**: ~140 MB RAM — native egui rendering, far lighter than browser/Electron apps
- **Pure native rendering**: Rust + [egui](https://github.com/emilk/egui); no WebView, no browser engine, no framework
- **One-click install**: double-click `install.bat`, then right-click any `.md` file → **Open with** → mdpeek

## Supported syntax

Headings, bold/italic, inline code, code blocks, blockquotes, ordered/unordered lists, task lists, tables, horizontal rules, links, images (relative paths resolve against the file's directory), GitHub-style callouts.

## Building (for developers)

```bash
cargo build --release
# Output: target/release/reader.exe
```

Requires the Rust `x86_64-pc-windows-gnu` target plus mingw binutils (which provides `dlltool`, used to generate the Windows API-set import libraries). The linker is `rust-lld`, bundled with rustup. The paths in `reader/.cargo/config.toml` are absolute paths for the build machine — adjust them to your local toolchain.

> Most users don't need to build anything: just grab `dist/mdpeek.exe` and double-click `dist/install.bat`.

## Install

```bash
# After copying the built binary into dist/ as mdpeek.exe:
dist\install.bat
```

Uninstall: `dist\uninstall.bat` (removes only the current user's registry entries; does not touch the system).

## Usage

- Right-click any `.md` file in Explorer → **Open with** → mdpeek
- Or from the command line: `mdpeek.exe doc.md`
- Or drag & drop a `.md` file onto the window

## Technical notes

- Built for `x86_64-pc-windows-gnu`, linked in self-contained mode with rust-lld — no mingw gcc required
- CJK text: auto-loads Microsoft YaHei from `C:\Windows\Fonts` at runtime
- Writes only to the `HKCU` registry; no administrator rights needed; does not change the `.md` default association
