# LAST-OFF: Medical Code Navigator

**Find TODOs, FIXMEs, and healthcare risks in your code - Made for beginners!**

![Demo](demo.gif) *(You can add a demo video/gif later)*

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

# Quick Start (For Beginners)

## Installation & Usage

| Step | Command | What it does |
|------|---------|--------------|
| **1. Clone** | `git clone https://github.com/NasreenAzeemi/last-off.git` | Download LAST-OFF |
| **2. Build** | `cd last-off && cargo build --release` | Compile the program |
| **3. Run** | `./target/release/last-off` | Scan your project |

## Step 1: Download LAST-OFF
```bash
git clone https://github.com/YOUR-USERNAME/last-off-simple.git
cd last-off-simple/last-off
```
## Step 2: Build the Program
```bash
cargo build --release
```
## Step 3: Use It!
```bash
# Go to your project folder
cd ~/my-healthcare-project

# Run LAST-OFF
~/last-off-simple/last-off/target/release/last-off
```

## Example Output

```bash
$ cd ~/my-healthcare-project
$ ~/last-off/target/release/last-off

============================
   LAST-OFF: Medical Code Navigator   
============================
📁 Scanning: /home/user/my-healthcare-project

┌────┬────────────┬────────────────────┬──────┬────────────────────────────────────┐
│ #  │ Type       │ File               │ Line │ Content                            │
├────┼────────────┼────────────────────┼──────┼────────────────────────────────────┤
│ 1  │ 🚨 SSN     │ src/database.rs    │ 42   │ ssn = "123-45-6789"                │
│ 2  │ TODO       │ src/main.rs        │ 15   │ // TODO: Add validation            │
│ 3  │ FIXME      │ src/utils.rs       │ 7    │ // FIXME: This leaks memory        │
└────┴────────────┴────────────────────┴──────┴────────────────────────────────────┘

📊 Summary:
  • 1 critical healthcare risk 🚨
  • 1 TODO 📝
  • 1 FIXME ⚠️
  • 3 total items to review

🎯 JUMP TO CODE:
  • Enter number (1-3) to select item
  • Press Enter to exit
  • Type 'a' to see ALL locations

Select item: 1
```

# How to Use (Step-by-Step)
## 1. Find Problems
LAST-OFF automatically scans and shows all issues.

## 2. Jump to Code
- Type the number (like `1`) and press Enter
- See the code around the problem
- Choose an editor to open it

## 3. Fix the Problem
- **Option 1:** Use **Nano** (easy for beginners)
- **Option 2:** Use **Gedit** (like Notepad for Linux)
- **Option 3:** Copy the command to open in your favorite editor

# What Does It Look For?
## 🚨 Critical Healthcare Risks (RED)
- `SSN` or `social security` - Social Security Numbers
- `patient_id` or `MRN` - Patient Medical Record Numbers
- `PHI` - Protected Health Information
- `DOB` - Date of Birth

## ⚠️ Warnings (YELLOW)
- `FIXME` - Things that need fixing
- `TODO` - Things to do later
- `XXX` - Problem areas
- `HACK` - Temporary solutions

## 🛠️ Editor Options
LAST-OFF works with these editors:

| Editor | Good For | How to Install |
|--------|----------|----------------|
| Nano | Beginners (easiest) | `sudo apt install nano` |
| Gedit | Like Windows Notepad | `sudo apt install gedit` |
| VS Code | Professional developers | [Download from website](https://code.visualstudio.com/download) |
| Vim | Experts | `sudo apt install vim` |

Don't have an editor installed? LAST-OFF will tell you exactly how to install it!

## 📁 File Types Scanned
- .rs (Rust files)
- .py (Python files)
- .js (JavaScript files)
- .java (Java files)
- .cpp (C++ files)
- And many more!
- Skipped files: Images (`.png`, `.jpg`), PDFs, ZIP files, build folders

## 🧪 Example: Finding a Patient ID Leak
Imagine this code in `database.rs`:
```rust

// BAD: This exposes patient data!
let patient_id = "P123456789";  // 🚨 PATIENT_ID found here!
```
**LAST-OFF will:**
1. Find this line
2. Show it in red as **🚨 PATIENT_ID**
3. Let you jump to line 42
4. Help you open the file to fix it

# Frequently Asked Questions
## 🤔 **Do I need Rust knowledge?**
**No!** Install Rust once, then just run `last-off`. No coding required!

## 🔐 Is my code safe?
**100% safe!** LAST-OFF runs locally, never uploads or stores your code.

## 🔎 What does it scan?
Current folder + 3 levels deep. Skips binaries and build folders.

## ⏰ How fast is it?
~1000 files in <5 seconds. Very fast!

## 💻 Windows/Mac support?
**Linux/Ubuntu only** (use Virtual Machine or WSL)

## 🛠 How to exit Nano?
Press **Ctrl+X** → **Y** (save) or **N** (discard) → **Enter**

## 🔧 "Command not found" error
You need to either:

- Use the full path: `~/last-off-simple/last-off/target/release/last-off`
- **OR** install globally: `cargo install --path .` from inside the `last-off` folder

## ❌ It's not finding my TODOs!
Make sure your comments have `TODO` or `FIXME` in them (case doesn't matter):
```rust

// TODO: Add error handling  ✓ Will be found
// todo: fix this later      ✓ Will be found  
// This needs work           ✗ Won't be found
```

# Found a Bug or Have an Idea?
- Found a bug or have an idea? [Open an Issue](https://github.com/NasreenAzeemi/last-off/issues)!
- Want to help improve the code? Fork the repository and submit a Pull Request.
- All suggestions are welcome and reviewed carefully.
>**⚠ Important:** Always check your code for sensitive data before sharing!
