# 🏥 LAST-OFF: Medical Code Navigator

**Find TODOs, FIXMEs, and healthcare risks in your code - Made for beginners!**

![Demo](demo.gif) *(You can add a demo video/gif later)*

## 🎯 What Does This Tool Do?

Imagine you have a healthcare app with 100 files. You need to find:
- **TODOs** (things to do later)
- **FIXMEs** (things that need fixing)
- **Healthcare risks** (SSN numbers, patient IDs, PHI data)

**LAST-OFF** scans ALL your code and shows you everything in one simple list!

## 🚀 Quick Start (For Beginners)

### Step 1: Install Rust (if you don't have it)
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# Press '1' and Enter when asked
source "$HOME/.cargo/env"
```

### Step 2: Download LAST-OFF
```bash
git clone https://github.com/YOUR-USERNAME/last-off-simple.git
cd last-off-simple/last-off
```
### Step 3: Build the Program
```bash
cargo build --release
```
### Step 4: Use It!
```bash
# Go to your project folder
cd ~/my-healthcare-project

# Run LAST-OFF
~/last-off-simple/last-off/target/release/last-off
```
### 📋 What You'll See
```bash
text
============================
   LAST-OFF: Medical Code Navigator   
============================
📁 Scanning: /home/you/my-project

#  Type        File              Line  Content
1  🚨 SSN      src/database.rs   42    ssn = "123-45-6789"
2  TODO        src/main.rs       15    // TODO: Add validation
3  FIXME       src/utils.rs      7     // FIXME: This leaks memory

## 📊 Summary:
  • 1 critical healthcare risks
  • 2 TODOs
  • 1 FIXMEs
  • 4 total items to review
```
## 🎮 How to Use (Step-by-Step)
### 1. Find Problems
LAST-OFF automatically scans and shows all issues.

### 2. Jump to Code
Type the number (like 1) and press Enter

See the code around the problem

Choose an editor to open it

### 3. Fix the Problem
Option 1: Use Nano (easy for beginners)

Option 2: Use Gedit (like Notepad for Linux)

Option 3: Copy the command to open in your favorite editor

## 🔍 What Does It Look For?
### 🚨 Critical Healthcare Risks (RED)
- SSN or social security - Social Security Numbers
- patient_id or MRN - Patient Medical Record Numbers
- PHI - Protected Health Information
- DOB - Date of Birth

⚠️ Warnings (YELLOW)
FIXME - Things that need fixing

TODO - Things to do later

XXX - Problem areas

HACK - Temporary solutions

🛠️ Editor Options
LAST-OFF works with these editors:

Editor	Good For	How to Install
Nano	Beginners (easiest)	sudo apt install nano
Gedit	Like Windows Notepad	sudo apt install gedit
VS Code	Professional developers	Download from website
Vim	Experts	sudo apt install vim
Don't have an editor installed? LAST-OFF will tell you exactly how to install it!

📁 File Types Scanned
.rs (Rust files)

.py (Python files)

.js (JavaScript files)

.java (Java files)

.cpp (C++ files)

And many more!

Skipped files: Images (.png, .jpg), PDFs, ZIP files, build folders

🧪 Example: Finding a Patient ID Leak
Imagine this code in database.rs:

rust
// BAD: This exposes patient data!
let patient_id = "P123456789";  // 🚨 PATIENT_ID found here!
LAST-OFF will:

Find this line

Show it in red as 🚨 PATIENT_ID

Let you jump to line 42

Help you open the file to fix it

❓ Common Questions
Q: I'm getting "command not found" for last-off
A: You need to either:

Use the full path: ~/last-off-simple/last-off/target/release/last-off

OR install globally: cargo install --path . from inside the last-off folder

Q: How do I exit Nano editor?
A: Press Ctrl+X (hold Ctrl, press X)

Q: Can I use this on Windows/Mac?
A: Currently Linux/Ubuntu only, but you can run it in a Virtual Machine

Q: It's not finding my TODOs!
A: Make sure your comments have TODO or FIXME in them (case doesn't matter):

rust
// TODO: Add error handling  ✓ Will be found
// todo: fix this later      ✓ Will be found  
// This needs work           ✗ Won't be found
🐛 Found a Bug or Have an Idea?
Go to our GitHub page

Click "Issues"

Click "New Issue"

Tell us what happened!

👏 Credits
Made with ❤️ for healthcare developers who want to keep patient data safe.

Remember: Always check your code for sensitive data before sharing!
