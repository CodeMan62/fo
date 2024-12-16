# File Organizer CLI

A simple, configurable command-line tool to organize files in directories based on their types. It supports dry-run operations, custom configurations, and flexible file categorization.

## **Features**

- Organize files into categorized folders.
- Supports custom configurations via a JSON file.
- Dry-run mode to preview changes without applying them.

## **Usage**

1. Build the tool:

```Bash
cargo build --release
```
1. Run the tool:

```Bash
    ./fo --path <directory> --dry-run --config <config.json>
```

    
3. Default configuration categorizes files into "Images," "Videos," etc. Customize `config.json` to modify this.

## **Example**

```bash
./fo --path ./Downloads --dry-run
```

---

### **To-Do List**

- [x]  Basic CLI implementation with `clap`.
- [x]  Directory scanning and file categorization.
- [x]  Dry-run functionality.
- [x]  Configurable JSON support.
- [ ]  Add support for symbolic links.
- [ ]  Include a progress bar for large directories.
- [ ]  Generate logs for each operation.
- [ ]  Support multi-threading for faster file operations.
- [ ]  Add tests for edge cases (e.g., empty directories).

Keep this as a growing list to enhance your project over time.
