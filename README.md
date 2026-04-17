# Audiobookshelf CLI

A powerful, asynchronous Rust-based command-line interface for interacting with your [Audiobookshelf](https://www.audiobookshelf.org/) server.

## Features

- **Authenticated Access**: Securely connect using your Audiobookshelf API key.
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
./target/release/audiobookshelf-cli --help
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
audiobookshelf-cli ping
```

**Explore Libraries**
```bash
audiobookshelf-cli libraries list
```

**View Your Profile**
```bash
audiobookshelf-cli me
```

**List Genres**
```bash
audiobookshelf-cli metadata genres
```

## Commands

| Command | Description |
|---------|-------------|
| `ping` | Verifies server reachability and credentials. |
| `libraries list` | Lists all libraries on the server. |
| `items list <ID>` | Lists all items in a specific library. |
| `authors list` | Lists all authors. |
| `playlists list` | Lists all playlists. |
| `series list` | Lists all series. |
| `metadata tags` | Lists all system tags. |
| `metadata genres` | Lists all system genres. |
| `users list` | Lists all users (requires admin permissions). |

## License

MIT
