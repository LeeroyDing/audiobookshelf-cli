# Audiobookshelf CLI (abs)

A powerful, asynchronous Rust-based command-line interface for interacting with your [Audiobookshelf](https://www.audiobookshelf.org/) server.

## Features

- **Authenticated Access**: Securely connect using your Audiobookshelf API key.
- **Human-Readable Tables**: Clean, formatted output for all list commands.
- **Scripting Friendly**: Use the `--json` global flag to get raw data for automation.
- **Cross-Platform**: Blazing fast performance on Linux, macOS, and Windows.
- **Rich Catalog Exploration**:
  - List and view details for **Libraries**, **Authors**, **Playlists**, and **Series**.
  - Retrieve detailed metadata for specific **Items**.
  - List all server-wide **Tags** and **Genres**.
- **User Management**: View your own profile and list system users.

## Quick Start

### 1. Installation

#### From Source
Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed, then:
```bash
git clone https://github.com/LeeroyDing/audiobookshelf-cli.git
cd audiobookshelf-cli
cargo build --release
./target/release/abs --help
```

### 2. Configuration

The CLI uses environment variables for authentication. Create a `.env` file in your working directory:

```env
AUDIOBOOKSHELF_SERVER_URL=https://your-abs-server.com
AUDIOBOOKSHELF_API_KEY=your_api_key_here
```

> [!TIP]
> You can find your API key in Audiobookshelf under **Settings > Users > [Your User] > API Key**.

### 3. Usage Examples

**Test Connection**
```bash
abs ping
```

**Explore Libraries (Table View)**
```bash
abs libraries list
```

**Get Raw JSON for Scripting**
```bash
abs --json libraries list | jq '.[0].id'
```

**View Your Profile**
```bash
abs me
```

## Global Options

| Option | Description |
|--------|-------------|
| `-j, --json` | Output raw JSON instead of tables. |
| `-h, --help` | Show help information. |
| `-V, --version` | Show version information. |

