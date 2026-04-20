# abs: Audiobookshelf CLI

A powerful, asynchronous Go-based command-line interface for interacting with your [Audiobookshelf](https://www.audiobookshelf.org/) server.

## 🚀 Features

- **Secure Authentication**: Store your API key in the system's secure keyring.
- **Discovery & Search**: Global search across all libraries, authors, and series.
- **Library Management**: Trigger server-side library scans directly from your terminal.
- **Item Management**: Update metadata, tags, and genres (including bulk operations).
- **Human-Readable Tables**: Clean, formatted output for all list and search commands.
- **Scripting Friendly**: Global `--json` flag for raw data output.
- **Shell Autocompletion**: Support for Bash, Zsh, Fish, and PowerShell.

## 📦 Installation

### Pre-built Binaries
Check the [Releases](https://github.com/LeeroyDing/audiobookshelf-cli/releases) page for pre-compiled binaries for Linux, macOS, and Windows.

### From Source
Ensure you have [Go](https://go.dev/doc/install) installed:
```bash
git clone https://github.com/LeeroyDing/audiobookshelf-cli.git
cd audiobookshelf-cli
go install .
```

## 🔐 Configuration & Authentication

`abs` supports three layers of configuration:

1. **System Keyring (Recommended)**:
   ```bash
   abs auth login --api-key <YOUR_KEY>
   ```
2. **Environment Variables**:
   `AUDIOBOOKSHELF_SERVER_URL` and `AUDIOBOOKSHELF_API_KEY`.
3. **Config File**:
   `~/.config/abs/config.yaml` on Linux/macOS.

## 🛠 Usage Examples

### Finding Content
**Global Search**
```bash
abs search "Sandman"
```

**List Books in a Library**
```bash
abs items list <LIBRARY_ID>
```

### Management
**Update Metadata**
```bash
abs items update <ITEM_ID> --tags "Sci-Fi, Space" --genres "Adventure"
```

**Bulk Tagging**
```bash
abs items bulk-update "ID1,ID2,ID3" --tags "Read"
```

**Trigger Library Scan**
```bash
abs libraries scan <LIBRARY_ID>
```

**Upload a Book**
```bash
abs upload "path/to/book.m4b" --library <LIBRARY_ID> --title "My Audiobook" --author "Author Name"
```

### Administration
**Server Information**
```bash
abs info
```

**Check Auth Status**
```bash
abs me
```

## 🐚 Shell Completions

Generate completion scripts for your shell:

```bash
# Zsh (add to ~/.zshrc)
source <(abs completion zsh)

# Bash (add to ~/.bashrc)
source <(abs completion bash)
```

## 📜 Commands Reference

Run `abs --help` or `abs <COMMAND> --help` for detailed information on every subcommand.

