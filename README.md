# loggerHeads

> Cross-platform system activity logger and watcher built in Rust.

loggerHeads is a lightweight, extensible system monitoring tool for developers, sysadmins, and power users. It logs system-level activities in real-time including file changes, process activity, USB events, and network interface updates.

---

## Features

### 🔍 File Watcher

* Watches directories recursively
* Detects and logs:

  * File creation
  * Modification
  * Deletion
  * Access

### ⚙️ Process Watcher

* Monitors running processes
* Logs:

  * CPU usage (with optional threshold filter)
  * Memory usage
  * Process command
  * Start time
  * Owning user

### 🔌 USB Watcher

* Detects and logs:

  * Device insertions
  * Device removals

### 🌐 Network Watcher

* Logs network interface changes:

  * Interface up/down
  * IP address changes

### 📅 Flexible Logging

* Supports multiple formats:

  * Plaintext
  * JSON
  * CSV

### 🔧 Cross-Platform Support

* Linux
* macOS
* Windows

---

## Getting Started

### Prerequisites

* Rust (latest stable)
* Cargo

### Build and Run

```bash
# Clone repo
$ git clone https://github.com/yourname/loggerHeads.git
$ cd loggerHeads

# Run (debug mode)
$ cargo run

# Build for release
$ cargo build --release
$ ./target/release/loggerHeads
```

---

<!-- ## Directory Structure

```
loggerHeads/
├── src/
│   ├── config/          # Config parsing and management
│   ├── logger/          # Logging utilities
│   ├── platform/        # OS-specific code
│   ├── watchers/        # Event watchers (file, process, usb, network)
│   └── main.rs
├── logs/                # Output logs directory
├── filewatcher.event.sh # Bash script to simulate file activity
├── Cargo.toml
```

---

## File Watcher Test

```bash
chmod +x filewatcher.event.sh
./filewatcher.event.sh
```

This script will simulate file creation, editing, and deletion in the monitored folder.

---

## Sample Log Output

```
[2025-06-01T14:52:12][FileChange] File created at /tmp/watch_test/sample.txt
[2025-06-01T14:52:20][ProcessEvent] Process: bash | PID: 12345 | CPU: 3.2% | MEM: 2.5MB | User: bunmi | Started: 2025-06-01T14:50:03 | Command: bash
[2025-06-01T14:53:01][USBEvent] USB device connected: Logitech USB Receiver
[2025-06-01T14:54:08][NetworkEvent] Interface eth0 now has IP: 192.168.1.23
```

--- -->

## Roadmap

* [x] File watcher
* [x] Process monitor
* [x] USB event listener
* [x] Network interface monitoring
<!-- * [ ] CLI configuration with `clap`
* [ ] Export to cloud storage or dashboard
* [ ] GUI/Web dashboard frontend
* [ ] Log rotation support -->

---

## License

MIT License

---

## Contributing
Bunmi

<!-- PRs, issues, and feedback welcome.
Join the mission to make system event monitoring accessible and robust across platforms. -->
