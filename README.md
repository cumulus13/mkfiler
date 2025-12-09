# mkfile - Advanced File Creator

A powerful command-line utility for creating blank files with support for brace expansion, automatic directory creation, desktop notifications, and clipboard integration.

## Features

- ✨ **Brace Expansion**: Create multiple files with a single command
- 📁 **Auto Directory Creation**: Automatically creates parent directories if they don't exist
- 📋 **Clipboard Integration**: Copies absolute file paths to clipboard
- 🔔 **Desktop Notifications**: Optional Growl notification support
- 🖥️ **Cross-Platform**: Works on Windows, Linux, and macOS
- 🎯 **Smart Parsing**: Handles complex patterns with spaces and multiple items

## Installation

```bash
cargo install mkfiler
```

## Usage

### Basic Syntax

```bash
mkfile [OPTIONS] FILE [FILE ...]
```

### Examples

#### Create Single File
```bash
mkfile test.txt
```

#### Create Multiple Files
```bash
mkfile file1.txt file2.rs file3.js
```

#### Create Files with Directory Structure
```bash
mkfile src/main.rs tests/test_main.rs
# Creates: src/ and tests/ directories automatically
```

#### Brace Expansion - Simple
```bash
mkfile test/{a,b,c}.txt
# Creates: test/a.txt, test/b.txt, test/c.txt
```

#### Brace Expansion - Directory Prefix
```bash
mkfile dotenv{core.rs,utils.rs}
# Creates: dotenv/ dotenv/core.rs, dotenv/utils.rs
```

#### Brace Expansion - With Spaces (Flexible Syntax)
```bash
mkfile src{main.rs, utils.rs config.rs}
# Creates: src/main.rs, src/utils.rs, src/config.rs
# Note: Handles both comma and space separation inside braces
```

#### Complex Project Structure
```bash
mkfile src/{main.rs,utils.rs} tests/{test_main.rs} README.md setup.rs requirements.txt
```

#### Real-World Example
```bash
mkfile mypackage/{core.rs,exceptions.rs,utils.rs} tests/{test_core.rs} setup.rs Cargo.toml README.md LICENSE requirements.txt .gitignore
```

## Brace Expansion Syntax

### Supported Formats

1. **Standard format** (recommended):
   ```bash
   mkfile dir/{file1,file2,file3}
   ```

2. **Directory prefix** (no slash):
   ```bash
   mkfile dir{file1,file2,file3}
   # Automatically adds '/' → dir/file1, dir/file2, dir/file3
   ```

3. **With spaces** (flexible):
   ```bash
   mkfile dir{file1, file2, file3}
   mkfile dir{file1 file2 file3}
   # Both work! Splits by comma AND spaces
   ```

4. **With file extensions**:
   ```bash
   mkfile src/{main,utils,config}.rs
   # Creates: src/main.rs, src/utils.rs, src/config.rs
   ```

### Brace Expansion Rules

- Items inside `{}` can be separated by:
  - Commas: `{a,b,c}`
  - Spaces: `{a b c}`
  - Both: `{a, b, c}` or `{a, b c}`
- If prefix doesn't end with `/` or `\`, it's treated as a directory name
- Empty items are skipped
- Nested braces are not supported

## Command-Line Options

### `--version`
Show version information

```bash
mkfile --version
```

### `--debug`
Enable detailed error messages and stack traces

```bash
mkfile --debug file1.txt file2.txt
```

### `--help`
Show help message with examples

```bash
mkfile --help
```

## Output

### Success
```
✓ File created: "C:\PROJECTS\myproject\src\main.rs"
✓ File created: "C:\PROJECTS\myproject\src\utils.rs"

2/2 file(s) created successfully
```

### With Errors
```
✓ File created: "valid.txt"
✗ Error creating file "invalid/path/file.txt": [Errno 13] Permission denied

1/2 file(s) created successfully
```

## Features in Detail

### Automatic Directory Creation

The script automatically creates all parent directories:

```bash
mkfile deep/nested/path/to/file.txt
# Creates: deep/ → deep/nested/ → deep/nested/path/ → deep/nested/path/to/
```

### Clipboard Integration

After creating each file, its absolute path is automatically copied to your clipboard (requires `clipboard` module).

### Desktop Notifications

If Growl is installed and running, you'll receive desktop notifications for each file created.

**Note**: The "Could not initialize Growl" warning is normal if Growl is not running. The script continues to work perfectly without it.

## Troubleshooting

### "Could not initialize Growl" Warning

This is **normal** and **harmless**. It means Growl notification service isn't running. The script works fine without it.

To suppress this warning, you can:
- Install and run Growl
- Or ignore it (doesn't affect functionality)

### Permission Denied Errors

If you get permission errors:
- Check directory permissions
- Run with appropriate privileges
- Use `--debug` flag for more details

### Files Not Created in Expected Directory

Check your brace expansion syntax:
```bash
# WRONG - No directory separator
mkfile mydir{file1,file2}      # Creates: mydir/file1, mydir/file2 ✓

# If you want nested directories, use /
mkfile mydir/{subdir/file1,file2}  # Creates: mydir/subdir/file1, mydir/file2
```

## Tips & Best Practices

1. **Use quotes for files with spaces**:
   ```bash
   mkfile "my file.txt"
   ```

2. **Combine with other commands**:
   ```bash
   mkfile src/{main,utils}.rs && code src/main.rs
   ```

3. **Create project templates**:
   ```bash
   # Save as a shell script
   mkfile project/{src,tests,docs}/{__init__.rs} README.md setup.rs
   ```

4. **Verify with tree command**:
   ```bash
   mkfile test/{a,b,c}.txt && tree test/
   ```

## Version History

### v2.0
- Added smart brace expansion parsing
- Auto directory creation
- Support for space-separated items in braces
- Improved error handling
- Better cross-platform support
- Class-based architecture

### v1.0
- Initial release
- Basic file creation
- Growl notifications

## License

Free to use and modify.

## Contributing

Feel free to submit issues and enhancement requests!

---

**Quick Reference**:
```bash
# Single file
mkfile file.txt

# Multiple files
mkfile file1 file2 file3

# With directories
mkfile src/main.rs tests/test.rs

# Brace expansion
mkfile src/{main,utils,config}.rs

# Complex structure
mkfile mypackage/{__init__,core,utils}.rs tests/{__init__,test_core}.rs README.md
```


## Author
[Hadi Cahyadi](mailto:cumulus13@gmail.com)
    

[![Buy Me a Coffee](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://www.buymeacoffee.com/cumulus13)

[![Donate via Ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/cumulus13)

[Support me on Patreon](https://www.patreon.com/cumulus13)