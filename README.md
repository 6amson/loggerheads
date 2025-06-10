# 🪵 LoggerHeads

**LoggerHeads** is a blazing-fast, cross-platform system monitoring and logging tool written in Rust. It monitors file changes, processes, network activity, and even raw packets — all configurable and logged in real-time. Perfect for auditing, security monitoring, or diagnostics.

---

## 📦 Download

Get the latest release from the [Releases Page](https://github.com/6amson/loggerheads/releases/latest).

| Platform | Standalone Binary | Compressed Archive |
|----------|-------------------|-------------------|
| 🐧 Linux  | `loggerheads-linux` (2.71 MB)   | `loggerheads-linux.tar.gz` (1.04 MB)    |
| 🍎 macOS  | `loggerheads-macos` (2.15 MB)   | `loggerheads-macos.tar.gz` (911 KB)     |
| 🪟 Windows| `loggerheads.exe` (8.94 MB)     | `loggerheads-windows.zip` (2.77 MB)     |

### Additional Downloads Available:
- **Source Code**: Available as `.zip` or `.tar.gz` 
- **GPG Signatures**: Each binary/archive has a corresponding `.asc` signature file
- **SHA256 Checksums**: Provided for all files for integrity verification

> **💡 Tip**: Download the compressed archives to save bandwidth, or grab the standalone binaries for immediate use.

---

## ✅ Verify Signature (Optional but Recommended)

LoggerHeads binaries are GPG signed for security.

- **GPG Key ID**: `4A3629C90B57475B`
- **Email**: `damilolasamson.ds@gmail.com`
- **Fingerprint**: `FC03 DC68 96CC A0F7 9B56 06BC 4A36 29C9 0B57 475B`

### Verification Steps:
```bash
# Import the public key
gpg --keyserver keyserver.ubuntu.com --recv-keys 4A3629C90B57475B

# Verify (example for Linux)
gpg --verify loggerheads-linux.tar.gz.asc loggerheads-linux.tar.gz
```

---

## 🚀 Extract & Run

### 🐧 Linux
```bash
tar -xzf loggerheads-linux.tar.gz
chmod +x loggerheads-linux
./loggerheads-linux --help
```

### 🍎 macOS
```bash
tar -xzf loggerheads-macos.tar.gz
chmod +x loggerheads-macos
./loggerheads-macos --help
```

### 🪟 Windows (PowerShell)
```powershell
Expand-Archive -Path loggerheads-windows.zip -DestinationPath .
.\loggerheads.exe --help
```
Or just double-click `loggerheads.exe` to run.

---

## ⚙️ Command Line Usage

```bash
loggerheads-[platform] [OPTIONS]
```

### Options

| Option | Description |
|--------|-------------|
| `--log-path` | Path to store logs (default: `./logs`) |
| `--interval` | Monitoring interval in seconds (default: `10`) |
| `--cpu-threshold` | CPU usage % to trigger alerts |
| `--watcher-dir` | Directory to monitor for file changes |
| `--log-format` | Format to use: `json` or `text` |


> **💡 Tip**: The file watcher logs a high volume of events, especially if a process maintains open or frequent connections to watched directories (e.g., databases, browsers, dev servers). To avoid bloated logs: Choose a --watcher-dir with a narrow scope (e.g., /home/user/projects/loggerHeads instead of / or /home) Or ensure you have sufficient disk space if you’re monitoring a busy directory.

### Example
```bash
./loggerheads-linux \
  --interval 5 \
  --cpu-threshold 30 \
  --watcher-dir /tmp \
  --log-format json
```

---

## 🔍 Features

- 🧠 **Process Monitoring** — Track high-CPU processes
- 🗂️ **File System Watcher** — Log created, modified, and deleted files
- 🌐 **Network Watcher** — Monitor IP connections and traffic
- 📡 **Packet Sniffing (WIP)** — Capture and inspect raw network packets
- 🧾 **Flexible Logging** — Output logs in JSON or human-readable formats
- 🔒 **Signed Releases** — GPG signatures for all releases
- 🛠️ **Cross-platform Support** — Linux, macOS, and Windows
- 🧩 **TOML Config** — Load log directory from `config.toml`
- ⚡ **CLI Configuration** — All options configurable via flags

---

## 🧪 Build From Source (Optional)

```bash
git clone https://github.com/6amson/loggerheads.git
cd loggerheads
cargo build --release
```

Output binary will be in:
```
target/release/loggerheads
```

---

## 📜 License

MIT License © Damilola Samson

---

**Made with 🦀 using Rust — Monitor everything.**

## Contributing
Bunmi
