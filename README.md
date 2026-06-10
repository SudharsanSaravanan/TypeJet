# TypeJet

A high-fidelity, cross-platform keyboard simulation tool written in Rust with Python bindings. It simulates hardware-level keystrokes with realistic, human-like jittered typing delays, dwell times, and modifier key shifts.

## 🚀 Features

- **Cross-Platform Support**: Simulates native keystrokes on both **Linux** (using `/dev/uinput`) and **Windows** (using the Win32 `SendInput` API).
- **Realistic Jitter**: Configurable random character-to-character delays, key dwell times (how long a key is held down), and shift key hesitation.
- **Smart Formatting**: Optional `code` mode that automatically strips leading/trailing line indentation to prevent double-indentation issues with editor auto-formatting.
- **Clipboard Integration**: Automatically reads and types clipboard contents, or accepts raw text via CLI argument.

---

## 🛠️ Operating System Architecture

```mermaid
graph LR
    A[Python CLI / Script] --> B[typejet Binding]
    B --> C{Platform OS}
    C -->|Linux| D[/dev/uinput Device Node]
    C -->|Windows| E[Win32 SendInput API]
```

---

## 💻 Quick Start

### Windows Users
Please refer to the comprehensive [WINDOWS_GUIDE.md](WINDOWS_GUIDE.md) for step-by-step setup instructions, prerequisites, virtual environment guide, and usage configurations.

Quick execution:
```cmd
# Create virtual environment and install maturin + pyperclip
python -m venv .venv
.venv\Scripts\python.exe -m pip install maturin pyperclip

# Compile Rust extension and register it
.venv\Scripts\python.exe -m maturin develop

# Run keyboard simulation (paste text from clipboard)
typejet.bat
```

### Linux Users
Ensure you have access permission to `/dev/uinput` (e.g. via `udev` rules or running with elevated privileges).

1. Install `maturin` and `pyperclip` in your python environment.
2. Compile and install:
   ```bash
   maturin develop
   ```
3. Run the executable python script:
   ```bash
   ./typejet
   ```

---

## 📝 License

This project is licensed under the [MIT License](Cargo.toml).