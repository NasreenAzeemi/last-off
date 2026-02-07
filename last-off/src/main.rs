//before: if an editor is not installed, it opens a blank page.
// Use specific imports to avoid conflicts
use comfy_table::{Table, Color, Cell, Attribute};
use comfy_table::presets::UTF8_FULL;
use colored::Colorize;
use std::io::{self, Write};

fn main() {
    println!("{}", "=".repeat(60).blue());
    println!("{}", "   LAST-OFF: Medical Code Navigator   ".bold().cyan());
    println!("{}", "=".repeat(60).blue());
    
    let path = std::env::current_dir().expect("Failed to get current directory");
    println!("📁 Scanning: {}", path.display());
    
    match scan_todos_and_risks(&path) {
        Ok(items) => {
            if items.is_empty() {
                println!("\n✅ No TODOs, FIXMEs, or healthcare risks found!");
            } else {
                display_table(&items);
                interactive_jump(&items);
            }
        }
        Err(e) => {
            eprintln!("{} Error scanning: {}", "❌".red(), e);
        }
    }
}

#[derive(Debug, Clone)]
struct CodeItem {
    id: usize,
    file: String,
    line: usize,
    kind: String,  // "TODO", "FIXME", "SSN", "PATIENT_ID", etc.
    content: String,
}

fn scan_todos_and_risks(path: &std::path::Path) -> Result<Vec<CodeItem>, Box<dyn std::error::Error>> {
    let mut items = Vec::new();
    let mut id = 1;
    
    use walkdir::WalkDir;
    
    for entry in WalkDir::new(path)
        .max_depth(3)  // Limit depth for speed
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            let file_path = entry.path();
            let file_path_str = entry.path().to_string_lossy().to_string();
            let file_name = entry.path().file_name().unwrap_or_default().to_string_lossy();
            
            // Skip binary/large files
            if should_skip_file(&file_name) {
                continue;
            }
            
            if let Ok(content) = std::fs::read_to_string(file_path) {
                for (line_num, line) in content.lines().enumerate() {
                    let line_num = line_num + 1;
                    let trimmed = line.trim();
                    
                    // Check for healthcare risks
                    if let Some(kind) = detect_healthcare_risk(trimmed) {
                        items.push(CodeItem {
                            id,
                            file: entry.path().to_string_lossy().to_string(),
                            line: line_num,
                            kind: kind.to_string(),
                            content: trimmed.to_string(),
                        });
                        id += 1;
                    }
                    // Check for TODOs/FIXMEs (if not already added as risk)
                    else if let Some(kind) = detect_todo(trimmed) {
                        items.push(CodeItem {
                            id,
                            file: entry.path().to_string_lossy().to_string(),
                            line: line_num,
                            kind: kind.to_string(),
                            content: trimmed.to_string(),
                        });
                        id += 1;
                    }
                }
            }
        }
    }
    
    Ok(items)
}

fn detect_healthcare_risk(line: &str) -> Option<&'static str> {
    let lower = line.to_lowercase();
    
    if lower.contains("ssn") || lower.contains("social security") {
        Some("🚨 SSN")
    } else if lower.contains("patient_id") || lower.contains("patient id") || lower.contains("mrn") {
        Some("🚨 PATIENT_ID")
    } else if lower.contains("phi") || lower.contains("protected health") {
        Some("⚠️ PHI")
    } else if lower.contains("dob") || lower.contains("date of birth") {
        Some("⚠️ DOB")
    } else {
        None
    }
}

fn detect_todo(line: &str) -> Option<&'static str> {
    let lower = line.to_lowercase();
    
    if lower.contains("fixme") {
        Some("FIXME")
    } else if lower.contains("todo") {
        Some("TODO")
    } else if lower.contains("xxx") {
        Some("XXX")
    } else if lower.contains("hack") {
        Some("HACK")
    } else {
        None
    }
}

fn should_skip_file(filename: &str) -> bool {
    let lower = filename.to_lowercase();
    lower.ends_with(".png") || 
    lower.ends_with(".jpg") || 
    lower.ends_with(".pdf") ||
    lower.ends_with(".zip") ||
    lower.contains("target") ||  // Skip build directories
    lower.contains("node_modules")
}

fn display_table(items: &[CodeItem]) {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    
    table.set_header(vec![
        Cell::new("#").fg(Color::Cyan),
        Cell::new("Type").fg(Color::Yellow),
        Cell::new("File").fg(Color::Green),
        Cell::new("Line").fg(Color::Blue),
        Cell::new("Content").fg(Color::White),
    ]);
    
    for item in items {
        let type_cell = if item.kind.starts_with("🚨") {
            Cell::new(&item.kind).fg(Color::Red).add_attribute(Attribute::Bold)
        } else if item.kind.starts_with("⚠️") {
            Cell::new(&item.kind).fg(Color::Yellow).add_attribute(Attribute::Bold)
        } else if item.kind == "FIXME" {
            Cell::new(&item.kind).fg(Color::Red)
        } else if item.kind == "TODO" {
            Cell::new(&item.kind).fg(Color::Yellow)
        } else {
            Cell::new(&item.kind).fg(Color::Blue)
        };
        
        table.add_row(vec![
            Cell::new(item.id.to_string()),
            type_cell,
            Cell::new(&item.file),
            Cell::new(item.line.to_string()),
            Cell::new(item.content.chars().take(50).collect::<String>()),
        ]);
    }
    
    println!("\n{}\n", table);
    
    // Simple summary
    let healthcare_count = items.iter().filter(|i| i.kind.contains("🚨")).count();
    let warning_count = items.iter().filter(|i| i.kind.contains("⚠️")).count();
    let fixme_count = items.iter().filter(|i| i.kind == "FIXME").count();
    let todo_count = items.iter().filter(|i| i.kind == "TODO").count();
    
    println!("{}", "📊 Summary:".bold());
    if healthcare_count > 0 {
        println!("  • {} critical healthcare risks", healthcare_count.to_string().red());
    }
    if warning_count > 0 {
        println!("  • {} healthcare warnings", warning_count.to_string().yellow());
    }
    if fixme_count > 0 {
        println!("  • {} FIXMEs", fixme_count.to_string().red());
    }
    if todo_count > 0 {
        println!("  • {} TODOs", todo_count.to_string().blue());
    }
    println!("  • {} total items to review", items.len());
}

fn interactive_jump(items: &[CodeItem]) {
    println!("\n{}", "🎯 JUMP TO CODE:".cyan().bold());
    println!("  • Enter number (1-{}) to select item", items.len());
    println!("  • Press Enter to exit");
    println!("  • Type 'a' to see ALL locations");
    
    print!("\n{} ", "Select item:".green().bold());
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    
    if input.is_empty() {
        println!("{}", "Goodbye! 👋".cyan());
        return;
    }
    
    if input.eq_ignore_ascii_case("a") {
        println!("\n{}", "📍 ALL ITEMS LOCATIONS:".cyan().bold());
        for item in items {
            println!("  {} {}:{} - {}", 
                item.id.to_string().blue(),
                item.file.green(),
                item.line.to_string().blue(),
                item.content.chars().take(40).collect::<String>()
            );
        }
        return;
    }
    
    if let Ok(num) = input.parse::<usize>() {
        if num >= 1 && num <= items.len() {
            let item = &items[num - 1];
            show_editor_options(item);
        } else {
            println!("{} Please enter 1-{}", "❌".red(), items.len());
        }
    }
}

fn show_editor_options(item: &CodeItem) {
    println!("\n{}", "=".repeat(60).cyan());
    println!("{} {}", "SELECTED:".bold(), item.content);
    println!("{} {}:{}", "LOCATION:".bold(), item.file.green(), item.line.to_string().blue());
    
    // Get full path and show context
    let current_dir = std::env::current_dir().unwrap_or_default();
    let full_path = current_dir.join(&item.file);
    let full_path_str = full_path.to_string_lossy().to_string();
    
    // SHOW CONTEXT
    if let Ok(content) = std::fs::read_to_string(&full_path) {
        let lines: Vec<&str> = content.lines().collect();
        let start = if item.line > 3 { item.line - 3 } else { 1 };
        let end = std::cmp::min(item.line + 2, lines.len());
        
        println!("\n{}", "📄 CONTEXT:".bold());
        for i in start..=end {
            let prefix = if i == item.line { ">>> " } else { "    " };
            let line_content = lines.get(i-1).unwrap_or(&"");
            println!("{}{:4}: {}", prefix, i, line_content);
        }
    }
    
    println!("{}", "=".repeat(60).cyan());
    
    println!("\n{}", "📝 OPEN WITH:".bold());
    
    // Check which editors are available
    let code_exists = std::process::Command::new("which").arg("code").output().map(|o| o.status.success()).unwrap_or(false);
    let vim_exists = std::process::Command::new("which").arg("vim").output().map(|o| o.status.success()).unwrap_or(false);
    let nano_exists = std::process::Command::new("which").arg("nano").output().map(|o| o.status.success()).unwrap_or(false);
    let gedit_exists = std::process::Command::new("which").arg("gedit").output().map(|o| o.status.success()).unwrap_or(false);
    
    // Show ALL options, but indicate which are installed
    let mut option_number = 1;
    
    // Option 1: VS Code
    if code_exists {
        println!("  {}. {} - {}", option_number, "VS Code".blue(), format!("code --goto {}:{}", full_path_str, item.line).green());
    } else {
        println!("  {}. {} - {}", option_number, "VS Code".dimmed().strikethrough(), "(Not installed)".red());
    }
    option_number += 1;
    
    // Option 2: Vim
    if vim_exists {
        println!("  {}. {} - {}", option_number, "Vim (new window)".blue(), format!("vim +{} '{}'", item.line, full_path_str).green());
    } else {
        println!("  {}. {} - {}", option_number, "Vim".dimmed().strikethrough(), "(Not installed)".red());
    }
    option_number += 1;
    
    // Option 3: Nano
    if nano_exists {
        println!("  {}. {} - {}", option_number, "Nano (new window)".blue(), format!("nano +{} '{}'", item.line, full_path_str).green());
    } else {
        println!("  {}. {} - {}", option_number, "Nano".dimmed().strikethrough(), "(Not installed)".red());
    }
    option_number += 1;
    
    // Option 4: Gedit
    if gedit_exists {
        println!("  {}. {} - {}", option_number, "Gedit".blue(), format!("gedit +{} '{}'", item.line, full_path_str).green());
    } else {
        println!("  {}. {} - {}", option_number, "Gedit".dimmed().strikethrough(), "(Not installed)".red());
    }
    option_number += 1;
    
    // Option 5: Copy commands
    println!("  {}. {} - Copy all commands", option_number, "Manual".yellow());
    option_number += 1;
    
    // Option 6: Cancel
    println!("  {}. {} - Back to list", option_number, "Cancel".red());
    
    let total_options = option_number; // Should be 6
    
    print!("\n{} (1-{}): ", "Choose editor".green().bold(), total_options);
    io::stdout().flush().unwrap();
    
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice_num: usize = match choice.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("{} Please enter a number", "❌".red());
            return;
        }
    };
    
    match choice_num {
        1 => {
            // VS Code
            if !code_exists {
                println!("\n{}", "=".repeat(50).red());
                println!("{} {} IS NOT INSTALLED", "❌".red(), "VS Code".bold());
                println!("{}", "=".repeat(50).red());
                println!("\n{} VS Code needs to be downloaded separately:", "📦".yellow());
                println!("  1. {}", "Visit: https://code.visualstudio.com/".blue().underline());
                println!("  2. {}", "Download the .deb package".cyan());
                println!("  3. {}", "Install with: sudo dpkg -i <package>.deb".green());
                println!("  4. {}", "If dependencies missing: sudo apt --fix-broken install".green());
                println!("\n{} Please choose another editor or install VS Code first", "💡".cyan());
                
                // Ask if they want to choose another editor
                print!("\n{} Choose another editor now? (y/n): ", "🔄".yellow());
                io::stdout().flush().unwrap();
                let mut retry = String::new();
                io::stdin().read_line(&mut retry).unwrap();
                
                if retry.trim().eq_ignore_ascii_case("y") {
                    show_editor_options(item); // Show options again
                }
            } else {
                println!("\n{}", "🚀 Opening VS Code...".yellow());
                let command = format!("code --goto '{}':{}", full_path_str, item.line);
                let _ = std::process::Command::new("bash")
                    .arg("-c")
                    .arg(&command)
                    .spawn();
            }
        }
        2 => {
            // Vim
            if !vim_exists {
                show_simple_install_prompt("Vim", "vim", "sudo apt install vim", item);
            } else {
                println!("\n{}", "🚀 Opening Vim in NEW terminal window...".yellow());
                println!("{}", "💡 In NEW window: Press ESC then :q to exit".cyan());
                let command = format!("gnome-terminal -- bash -c \"vim +{} '{}'; exec bash\"", item.line, full_path_str);
                let _ = std::process::Command::new("bash")
                    .arg("-c")
                    .arg(&command)
                    .spawn();
            }
        }
        3 => {
            // Nano
            if !nano_exists {
                show_simple_install_prompt("Nano", "nano", "sudo apt install nano", item);
            } else {
                println!("\n{}", "🚀 Opening Nano in NEW terminal window...".yellow());
                println!("{}", "💡 Look for a NEW window to appear!".cyan());
                
                let command = format!("gnome-terminal -- bash -c \"nano +{} '{}'; exec bash\"", item.line, full_path_str);
                
                match std::process::Command::new("bash")
                    .arg("-c")
                    .arg(&command)
                    .spawn() {
                    Ok(_) => {
                        println!("✅ New terminal window launched!");
                        println!("{} Use arrow keys in the NEW window", "💡".cyan());
                        println!("{} Press Ctrl+X to exit Nano", "💡".cyan());
                    }
                    Err(e) => {
                        println!("{} Failed to open new window: {}", "❌".red(), e);
                        println!("{} Try this instead:", "👉".yellow());
                        println!("  {}", format!("nano +{} '{}'", item.line, full_path_str).green());
                        println!("  (Then press Ctrl+X to exit)");
                    }
                }
            }
        }
        4 => {
            // Gedit
            if !gedit_exists {
                show_simple_install_prompt("Gedit", "gedit", "sudo apt install gedit", item);
            } else {
                println!("\n{}", "🚀 Opening Gedit...".yellow());
                let command = format!("gedit +{} '{}'", item.line, full_path_str);
                let _ = std::process::Command::new("bash")
                    .arg("-c")
                    .arg(&command)
                    .spawn();
            }
        }
        5 => {
            // Copy all commands
            println!("\n{}", "📋 ALL COMMANDS:".cyan());
            println!("{} VS Code:", "💻".blue());
            println!("  code --goto '{}':{}", full_path_str, item.line);
            println!("{} Vim:", "💻".green());
            println!("  vim +{} '{}'", item.line, full_path_str);
            println!("  gnome-terminal -- bash -c \"vim +{} '{}'; exec bash\"", item.line, full_path_str);
            println!("{} Nano:", "💻".yellow());
            println!("  nano +{} '{}'", item.line, full_path_str);
            println!("  gnome-terminal -- bash -c \"nano +{} '{}'; exec bash\"", item.line, full_path_str);
            println!("{} Gedit:", "💻".magenta());
            println!("  gedit +{} '{}'", item.line, full_path_str);
            println!("\n{} Copy any command above and paste in terminal", "📋".cyan());
        }
        6 => {
            // Cancel
            println!("{}", "Returning to list...".yellow());
        }
        _ => {
            println!("{} Please enter 1-{}", "❌".red(), total_options);
        }
    }
}

// Helper function for simple apt-installable editors
fn show_simple_install_prompt(editor_name: &str, command_name: &str, install_cmd: &str, item: &CodeItem) {
    println!("\n{}", "=".repeat(50).red());
    println!("{} {} IS NOT INSTALLED", "❌".red(), editor_name.bold());
    println!("{}", "=".repeat(50).red());
    
    println!("\n{} To install {}:", "📦".yellow(), editor_name);
    println!("  {}", install_cmd.green());
    
    println!("\n{} Quick install now? (y/n): ", "⚡".cyan());
    io::stdout().flush().unwrap();
    
    let mut install_now = String::new();
    io::stdin().read_line(&mut install_now).unwrap();
    
    if install_now.trim().eq_ignore_ascii_case("y") {
        println!("\n{} Running: {}", "🔧".yellow(), install_cmd);
        println!("{} This may take a moment...", "⏳".cyan());
        
        match std::process::Command::new("sudo")
            .arg("apt")
            .arg("install")
            .arg("-y")
            .arg(command_name)
            .spawn() {
            Ok(mut child) => {
                let status = child.wait().expect("Failed to wait for apt install");
                if status.success() {
                    println!("{} {} installed successfully!", "✅".green(), editor_name);
                    println!("\n{} Run last-off again to use {}", "🔄".cyan(), editor_name);
                } else {
                    println!("{} Failed to install {}", "❌".red(), editor_name);
                }
            }
            Err(e) => {
                println!("{} Need sudo privileges. Run manually:", "🔒".red());
                println!("  {}", install_cmd);
            }
        }
    } else {
        println!("\n{} Please choose another editor or install {} first", "💡".cyan(), editor_name);
        
        print!("{} Choose another editor now? (y/n): ", "🔄".yellow());
        io::stdout().flush().unwrap();
        let mut retry = String::new();
        io::stdin().read_line(&mut retry).unwrap();
        
        if retry.trim().eq_ignore_ascii_case("y") {
            show_editor_options(item); // Show options again
        }
    }
}
