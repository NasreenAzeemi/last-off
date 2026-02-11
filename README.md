# LAST-OFF: Medical Code Navigator

**Find TODOs, FIXMEs, and healthcare risks in your code - Made for healthcare developers!**

# What Does This Tool Do?

Imagine you have a healthcare app with 100 files. You need to find:
- **TODOs** (things to do later)
- **FIXMEs** (things that need fixing)
- **Healthcare risks** (SSN numbers, patient IDs, PHI data)

**LAST-OFF** scans ALL your code and shows you everything in one simple list!

## Features

- **Automatic Scanning** - Finds TODOs, FIXMEs, and healthcare risks
- **Color-Coded Output** - Red for critical risks, yellow for warnings
- **Quick Navigation** - Jump to any line with one keystroke
- **Multiple Editor Support** - Works with Nano, Gedit, VS Code, Vim
- **Healthcare Focused** - Specifically looks for PHI, SSN, patient IDs
- **Beginner Friendly** - Simple commands, helpful error messages

# Quick Start

## Installation
```bash
# 1. Download
git clone https://github.com/NasreenAzeemi/last-off.git
cd last-off

# 2. Build
cargo build --release
```

## How to Run
**Option A: Run from project folder**
```bash
./target/release/last-off
```
**Option B: Install globally (recommended)** 
```bash
cargo install --path .
# Now you can use last-off from anywhere!
last-off
```

## Example Session
```rust
# 1. Go to your project
cd ~/projects/patient-management-system

# 2. Run LAST-OFF
last-off

==================================================================
   LAST-OFF: Medical Code Navigator   
==================================================================
SCANNING: /home/user/projects/patient-management-system

┌────┬─────────────────┬────────────────────┬──────┬────────────────────────────────────┐
│ #  │ Type            │ File               │ Line │ Content                            │
├────┼─────────────────┼────────────────────┼──────┼────────────────────────────────────┤
│ 1  │ [CRITICAL] SSN  │ src/database.rs    │ 42   │ ssn = "123-45-6789"                │
│ 2  │ TODO            │ src/main.rs        │ 15   │ // TODO: Add validation            │
│ 3  │ FIXME           │ src/utils.rs       │ 7    │ // FIXME: This leaks memory        │
└────┴─────────────────┴────────────────────┴──────┴────────────────────────────────────┘

SUMMARY:
  • 1 critical healthcare risk
  • 1 TODO
  • 1 FIXME
  • 3 total items to review

JUMP TO CODE:
  • Enter number (1-3) to select item
  • Press Enter to exit
  • Type 'all' to see all locations

Select item: 1
```

# How to Use
## 1. Scan Your Project
Simply run `lat-off` in any folder (gloablly installed). It automatically finds all issues. 

## 2. Jump to Code
- Type the number (like `1`) and press Enter
- View the code conetxt around the issue
- Choose an editor to open the file

## 3. Fix the Issue
LAST-OFF helps you open the exact line in your preferred editor:
| Editor | Command |
|---|---|
| Nano | `nano +42 file.rs` |
| Gedit | `gedit +42 file.rs` |
| VS Code | `code --goto file.rs +42` |
| Vim | `vim +42 file.rs` |

# What LAST-OFF Detects?
## 🚨 Critical Healthcare Risks
| Pattern | Example | Risk |
|---------|---------|------|
| SSN, social security | `ssn = "123-45-6789"` | **CRITICAL** |
| patiend_id, MRN | `patient_id = "P123456"` | **CRITICAL** |
| PHI, protected health | `phi_data = load()` | **CRITICAL** |
| DOB, date of birth | `dob = "01/01/1979"` | **CRITICAL** |
 
## ⚠️ Warnings (YELLOW)
| Pattern | Purpose |
|---------|---------|
| TODO | Incomplete code|
| FIXME | Known bugs |
| XXX | Problem areas |
| HACK | Temporary solutions |

## 🛠️ Editor Options
| Editor | Good For | How to Install |
|--------|----------|----------------|
| Nano | Beginners (easiest) | `sudo apt install nano` |
| Gedit | Like Windows Notepad | `sudo apt install gedit` |
| VS Code | Professional developers | [Download from website](https://code.visualstudio.com/download) |
| Vim | Experts | `sudo apt install vim` |

Don't have an editor installed? LAST-OFF will tell you exactly how to install it!

# File Types Scanned
- **Code Files:** `.rs`, `.py`, `.js`, `.java`, `.cpp`, `.go`, `.rb`, `.php`
- **Config Files:** `.toml`, `.yaml`, `.json`, `.xml`
- **Web Files:** `.html`, `.css`, `.jsx`, `.ts`
- **Skipped:** Images (`.png`, `.jpg`), PDFs, ZIPs, `target/`, `node_modules/`

# Frequently Asked Questions
### Do I need to know Rust?
**No.** You just need to install Rust once, After that `last-off` works like any other command-line tool.

### Is my code uploaded anywhere?
**Never.** LAST-OFF runs locally. Your code never leaves your machine.

### Why is the binary so small?
LAST-OFF is written in Rust and optimized for performance. The compiled binary is **~900KB.**

### What does it scan?
Current folder + 3 levels deep. Skips binaries and build folders.

### Windows/Mac support?
**Linux/Ubuntu only.** Use Virtual Machine or WSL (Window Subsystem for Linux).

### How do I update LAST-OFF?
```bash
cd last-off
git pull
cargo build --release
# or if installed globally:
cargo install --path . --force
```

### It says "command not found"!
#### Solution 1: Use the full path: 
```bash
~/last-off-simple/last-off/target/release/last-off
```

#### Solution 2: Install globally (do once): 
```bash
cd last-off
cargo install --path .
# Now just type last-off
```

### ❌ It's not finding my TODOs!
Make sure your comments have `TODO` or `FIXME` (case doesn't matter):
```rust

// TODO: Add error handling  ✓ Will be found
// todo: fix this later      ✓ Will be found  
// This needs work           ✗ Won't be found
```

### How do I exit Nano?
Press `Ctrl+X`, then `Y` to save or `N` to discard, then `Enter`.

# Contributing
Found a bug? Have an idea? [Open an Issue](https://github.com/NasreenAzeemi/last-off/issues)!

Want to contribute code? 
1. Fork the repository
2. Create a feature branch
3. Submit a pull request

All contributions are reviewed and welcome.
>**⚠ Important:** Always check your code for sensitive data before comitting or sharing!
