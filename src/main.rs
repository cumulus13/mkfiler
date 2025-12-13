// mkfile - Create blank files with notification support
// Cross-platform file creation utility with GNTP/Growl integration
//
// Author: cumulus13 (cumulus13@gmail.com)
// Version: 2.0

use regex::Regex;
use std::env;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::process;
use std::sync::Mutex;
use lazy_static::lazy_static;

use gntp::{GntpClient, NotificationType, Resource};

const NAME: &str = "mkfile";
const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHOR: &str = "cumulus13 (cumulus13@gmail.com)";

// Global GNTP client (initialized once)
lazy_static! {
    static ref GNTP_CLIENT: Mutex<Option<GntpClient>> = Mutex::new(None);
}

/// FileCreator handles file creation with notification support
struct FileCreator {
    icon_path: PathBuf,
    debug: bool,
    use_gntp: bool,
}

impl FileCreator {
    /// Creates a new FileCreator instance
    fn new(debug: bool, use_gntp: bool) -> Self {
        let exe_path = env::current_exe().unwrap_or_else(|_| PathBuf::from("."));
        let icon_path = exe_path
            .parent()
            .unwrap_or(Path::new("."))
            .join("mkfile.jpg");

        FileCreator {
            icon_path,
            debug,
            use_gntp,
        }
    }

    /// Initialize GNTP client (call once)
    fn init_gntp(&self) -> Result<(), String> {
        if !self.use_gntp {
            return Ok(());
        }

        let mut client_guard = GNTP_CLIENT.lock().unwrap();
        
        if client_guard.is_some() {
            return Ok(()); // Already initialized
        }

        let mut client = GntpClient::new(NAME);
        
        // Load application icon if exists
        if self.icon_path.exists() {
            match Resource::from_file(&self.icon_path) {
                Ok(icon) => {
                    client = client.with_icon(icon);
                }
                Err(e) => {
                    if self.debug {
                        eprintln!("Warning: Could not load icon: {:?}", e);
                    }
                }
            }
        }

        // Register notification type
        let notification = NotificationType::new("create")
            .with_display_name("File Created")
            .with_enabled(true);

        match client.register(vec![notification]) {
            Ok(_) => {
                *client_guard = Some(client);
                Ok(())
            }
            Err(e) => {
                if self.debug {
                    Err(format!("GNTP registration failed: {:?}", e))
                } else {
                    // Silent fail if GNTP not available
                    Ok(())
                }
            }
        }
    }

    /// Parse brace expansion patterns
    fn parse_brace_expansion(&self, text: &str) -> Vec<String> {
        let pattern = Regex::new(r"([^{]*)\{([^}]+)\}([^{]*)").unwrap();

        if let Some(caps) = pattern.captures(text) {
            let mut prefix = caps.get(1).map_or("", |m| m.as_str()).to_string();
            let items_str = caps.get(2).map_or("", |m| m.as_str());
            let suffix = caps.get(3).map_or("", |m| m.as_str());

            // Add separator if needed
            if !prefix.is_empty() && !prefix.ends_with('/') && !prefix.ends_with('\\') {
                prefix.push('/');
            }

            // Split and process items
            let parts: Vec<&str> = items_str.split(',').collect();
            let mut items = Vec::new();

            for part in parts {
                let sub_items: Vec<&str> = part.trim().split_whitespace().collect();
                items.extend(sub_items);
            }

            // Expand into individual files
            let mut expanded = Vec::new();
            for item in items {
                if !item.is_empty() {
                    let filepath = format!("{}{}{}", prefix, item, suffix);
                    expanded.push(filepath);
                }
            }

            expanded
        } else {
            vec![text.to_string()]
        }
    }

    /// Create a blank file
    fn create_file(&self, filepath: &str) -> bool {
        let path = Path::new(filepath);

        // Create parent directories if needed
        if let Some(parent) = path.parent() {
            if let Err(e) = fs::create_dir_all(parent) {
                eprintln!("✗ Error creating directory for \"{}\": {}", filepath, e);
                return false;
            }
        }

        // Create the file
        match File::create(path) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("✗ Error creating file \"{}\": {}", filepath, e);
                if self.debug {
                    eprintln!("   Debug: {:?}", e);
                }
                return false;
            }
        }

        // Get absolute path
        let abs_path = path
            .canonicalize()
            .unwrap_or_else(|_| path.to_path_buf())
            .display()
            .to_string();

        // Copy to clipboard (platform specific)
        self.copy_to_clipboard(&abs_path);

        // Send notification
        self.notify(filepath);

        println!("✓ File created: \"{}\"", abs_path);
        true
    }

    /// Copy text to clipboard (platform specific)
    fn copy_to_clipboard(&self, text: &str) {
        #[cfg(target_os = "windows")]
        {
            use std::process::Command;
            let _ = Command::new("cmd")
                .args(&["/C", &format!("echo {} | clip", text)])
                .output();
        }

        #[cfg(target_os = "macos")]
        {
            use std::process::Command;
            use std::io::Write;
            if let Ok(mut child) = Command::new("pbcopy")
                .stdin(std::process::Stdio::piped())
                .spawn() {
                if let Some(mut stdin) = child.stdin.take() {
                    let _ = stdin.write_all(text.as_bytes());
                }
            }
        }

        #[cfg(target_os = "linux")]
        {
            use std::process::Command;
            use std::io::Write;
            if let Ok(mut child) = Command::new("xclip")
                .args(&["-selection", "clipboard"])
                .stdin(std::process::Stdio::piped())
                .spawn() {
                if let Some(mut stdin) = child.stdin.take() {
                    let _ = stdin.write_all(text.as_bytes());
                }
            }
        }
    }

    /// Send notification
    fn notify(&self, filepath: &str) {
        if !self.use_gntp {
            return;
        }

        let filename = Path::new(filepath)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(filepath);

        let client_guard = GNTP_CLIENT.lock().unwrap();
        
        if let Some(ref client) = *client_guard {
            match client.notify(
                "create",
                NAME,
                &format!("File created: \"{}\"", filename)
            ) {
                Ok(_) => {},
                Err(e) => {
                    if self.debug {
                        eprintln!("Notification error: {:?}", e);
                    }
                }
            }
        }
    }

    /// Create multiple files
    fn create_files(&self, files: &[String]) -> usize {
        let mut success_count = 0;
        let mut all_files = Vec::new();

        // Expand all brace patterns
        for file_arg in files {
            if file_arg.contains('{') && file_arg.contains('}') {
                let expanded = self.parse_brace_expansion(file_arg);
                all_files.extend(expanded);
            } else {
                all_files.push(file_arg.clone());
            }
        }

        // Create all files
        for filepath in all_files {
            if self.create_file(&filepath) {
                success_count += 1;
            }
        }

        success_count
    }
}

/// Reconstruct file arguments
fn reconstruct_files(args: &[String]) -> Vec<String> {
    let joined = args.join(" ");

    let mut file_list = Vec::new();
    let mut current = String::new();
    let mut in_braces = false;

    for ch in joined.chars() {
        match ch {
            '{' => {
                in_braces = true;
                current.push(ch);
            }
            '}' => {
                in_braces = false;
                current.push(ch);
            }
            ' ' => {
                if !in_braces {
                    if !current.is_empty() {
                        file_list.push(current.clone());
                        current.clear();
                    }
                } else {
                    current.push(ch);
                }
            }
            _ => {
                current.push(ch);
            }
        }
    }

    if !current.is_empty() {
        file_list.push(current);
    }

    file_list
}

fn print_help() {
    println!("mkfile v{} - Create blank files with notification support", VERSION);
    println!("Author: {}\n", AUTHOR);
    println!("Usage: mkfile [OPTIONS] FILE...\n");
    println!("Options:");
    println!("  --help, -h         Show this help message");
    println!("  --version, -v      Show version information");
    println!("  --debug, -d        Show detailed error messages");
    println!("  --no-gntp          Disable GNTP/Growl notifications\n");
    println!("Examples:");
    println!("  mkfile file.txt                       # Create single file");
    println!("  mkfile file1.txt file2.py file3       # Create multiple files");
    println!("  mkfile dir/subdir/file.txt            # Create with directories");
    println!("  mkfile dir/{{a,b,c}}.txt                # Brace expansion");
    println!("  mkfile dotenv/{{__init__.py,core.py}}   # Create package structure");
    println!("\nNote: GNTP notifications require Growl for Windows or compatible client.");
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    // Parse flags
    let mut debug = false;
    let mut use_gntp = true;
    let mut files = Vec::new();

    for arg in &args {
        match arg.as_str() {
            "--help" | "-h" => {
                print_help();
                process::exit(0);
            }
            "--version" | "-v" => {
                println!("mkfile v{} by {}", VERSION, AUTHOR);
                process::exit(0);
            }
            "--debug" | "-d" => {
                debug = true;
            }
            "--no-gntp" => {
                use_gntp = false;
            }
            _ => {
                files.push(arg.clone());
            }
        }
    }

    if files.is_empty() {
        print_help();
        process::exit(0);
    }

    let file_list = reconstruct_files(&files);

    // Create FileCreator
    let creator = FileCreator::new(debug, use_gntp);
    
    // Initialize GNTP
    if let Err(e) = creator.init_gntp() {
        if debug {
            eprintln!("GNTP init error: {}", e);
        }
    }

    // Create files
    let success_count = creator.create_files(&file_list);

    // Calculate total expected
    let total_expected: usize = file_list
        .iter()
        .map(|f| {
            if f.contains('{') && f.contains('}') {
                creator.parse_brace_expansion(f).len()
            } else {
                1
            }
        })
        .sum();

    println!("\n{}/{} file(s) created successfully", success_count, total_expected);

    if success_count == total_expected {
        process::exit(0);
    } else {
        process::exit(1);
    }
}
