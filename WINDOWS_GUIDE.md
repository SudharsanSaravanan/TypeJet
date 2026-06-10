# TypeJet: Windows Installation & User Guide

This guide provides instructions for setting up, compiling, and running the `typejet` keyboard simulation utility on Microsoft Windows.

---

## 🚀 Architecture Overview

TypeJet uses a hybrid Rust/Python architecture to achieve near-instantaneous, hardware-level keyboard simulation. 

```mermaid
graph TD
    A[Clipboard / CLI Input] --> B[typejet_cli.py CLI Wrapper]
    B --> C[pyo3 Bindings]
    C --> D[typejet Rust Library]
    D --> E{Operating System}
    E -->|Windows| F[Win32 SendInput API]
    E -->|Linux| G[/dev/uinput Kernel Device]
    F --> H[Target Application Editor/Console]
    G --> H
```

---

## 📋 Prerequisites

To compile and run this project on Windows, you will need:

1. **Python 3.8+**: Ensure Python is added to your system `PATH`.
2. **Rust & Cargo**: Standard Rust toolchain (recommended to install via [rustup.rs](https://rustup.rs/)).
3. **C++ Build Tools**: Required by Rust's MSVC compiler (normally installed via Visual Studio Build Tools).

---

## ⚙️ Installation & Setup

Follow these steps to set up the virtual environment, compile the Rust extension, and prepare the project.

### 1. Create a Python Virtual Environment
Open PowerShell or Command Prompt in the project root and create a local virtual environment:
```powershell
python -m venv .venv
```

### 2. Install Development Dependencies
Install `maturin` (used to compile and package the Rust module for Python) and `pyperclip` (used to read from the system clipboard):
```powershell
.venv\Scripts\python.exe -m pip install maturin pyperclip
```

### 3. Build & Install the Rust Extension Module
Run Maturin to compile the Rust codebase and install it into your virtual environment:
```powershell
.venv\Scripts\python.exe -m maturin develop
```

This will automatically compile the Rust package, wrap it using `pyo3`, and install it under the name `typejet` in your virtual environment.

---

## ⌨️ Usage

On Windows, launcher scripts are provided for both CMD and PowerShell.

### Standard Execution (Clipboard Mode)
Copy any text to your clipboard, then run the utility. By default, it will wait for **3 seconds** to give you time to switch focus to your target text editor or input field.

**Using Command Prompt (CMD):**
```cmd
typejet.bat
```

**Using PowerShell:**
```powershell
.\typejet.ps1
```

---

## 🛠️ CLI Options Reference

The Windows launcher script supports all parameters of the underlying Python CLI wrapper:

| Argument | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `--mode` | `code` or `raw` | `code` | **`code`**: Strips leading and trailing whitespace from each line to prevent automatic IDE/editor indentation conflicts. <br>**`raw`**: Types text exactly as-is. |
| `--delay` | `float` | `3.0` | Initial countdown timer in seconds before typing starts. Use `0` to type immediately. |
| `--text` | `string` | `None` | Passes text directly via CLI argument instead of copying from the clipboard. |
| `--char-min` | `float` | `0.002` | Minimum delay in seconds between consecutive characters. |
| `--char-max` | `float` | `0.003` | Maximum delay in seconds between consecutive characters. |
| `--dwell-min`| `int` | `20` | Minimum key-down dwell time in milliseconds (how long keys are held). |
| `--dwell-max`| `int` | `60` | Maximum key-down dwell time in milliseconds. |
| `--shift-min`| `int` | `10` | Minimum delay in milliseconds for shift-key state changes. |
| `--shift-max`| `int` | `30` | Maximum delay in milliseconds for shift-key state changes. |

### Advanced Usage Examples

1. **Instant typing (zero delay):**
   ```powershell
   .\typejet.ps1 --delay 0 --char-min 0 --char-max 0 --dwell-min 0 --dwell-max 0
   ```

2. **Simulating slow, human-like typing speed (e.g. for testing inputs):**
   ```powershell
   .\typejet.ps1 --char-min 0.05 --char-max 0.15 --dwell-min 40 --dwell-max 80
   ```

3. **Passing direct text via CLI without clipboard:**
   ```powershell
   .\typejet.ps1 --text "Hello World!" --delay 1.5
   ```

4. **Raw typing mode (preserves original indentation):**
   ```powershell
   .\typejet.ps1 --mode raw
   ```

---

## 🔧 Troubleshooting & Tips

### 💡 Keystrokes Not Registering
- **Administrative Privileges**: If you are typing into a program running as Administrator (e.g., an elevated CMD/PowerShell terminal or registry editor), Windows will block `SendInput` events from standard user processes. Run your terminal or IDE as Administrator to allow simulated keyboard events to bypass User Account Control (UAC) constraints.
- **Wrong Window Focused**: Ensure you click into the target input field immediately after running the command during the 3-second countdown.

### 💡 Editor Double-Indentation
By default, some editors (like VS Code, IntelliJ, or Notepad++) auto-indent lines as you type. If you simulate pasting python or structured code, this can double the indentation. Always use `--mode code` (default) which strips whitespace from line beginnings, or disable auto-indentation temporarily in your editor settings.
