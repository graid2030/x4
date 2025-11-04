use std::fs;
use std::path::Path;

const MAX_LINES: usize = 250;

fn main() {
    println!("cargo:rerun-if-changed=src/");

    let mut violations = Vec::new();

    // Check all .rs files in src/
    check_directory("src", &mut violations);

    if !violations.is_empty() {
        eprintln!("\n❌ BUILD FAILED: Files exceed {} line limit (CLAUDE.md)", MAX_LINES);
        eprintln!("╭─────────────────────────────────────────────────────────╮");
        eprintln!("│ The following files violate the 250-line limit:        │");
        eprintln!("├─────────────────────────────────────────────────────────┤");

        for (file, lines) in &violations {
            eprintln!("│ {:48} {:>4} lines │", file, lines);
        }

        eprintln!("╰─────────────────────────────────────────────────────────╯");
        eprintln!("\n💡 Per CLAUDE.md: Break large files into smaller modules");
        eprintln!("   Max file size: 200-250 lines\n");

        std::process::exit(1);
    }

    println!("✓ All source files comply with 250-line limit");
}

fn check_directory(dir: &str, violations: &mut Vec<(String, usize)>) {
    let path = Path::new(dir);

    if !path.exists() {
        return;
    }

    let entries = match fs::read_dir(path) {
        Ok(entries) => entries,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();

        if path.is_dir() {
            check_directory(path.to_str().unwrap(), violations);
        } else if path.extension().and_then(|s| s.to_str()) == Some("rs") {
            if let Ok(content) = fs::read_to_string(&path) {
                let line_count = content.lines().count();

                if line_count > MAX_LINES {
                    let file_name = path.strip_prefix("src/")
                        .unwrap_or(&path)
                        .to_string_lossy()
                        .to_string();
                    violations.push((file_name, line_count));
                }
            }
        }
    }
}
