---
name: abs
description: "Commands for managing Audiobookshelf servers via the `abs` CLI. WHEN: \"search audiobooks\", \"upload book\", \"list library items\", \"update metadata\", \"scan library\", \"manage audiobookshelf\"."
license: MIT
metadata:
  author: Leeroy Ding
  version: "1.2.0"
---

# Audiobookshelf CLI (abs)

The `abs` CLI is a Go-based tool for interacting with Audiobookshelf servers.

## Setup & Authentication

- **Login**: `abs auth login --api-key <KEY>` (Stores key in system keyring)
- **Status**: `abs me` (Current user) or `abs info` (Server status)
- **Logout**: `abs auth logout`

## Content Discovery

- **Search**: `abs search "<QUERY>"` (Search across all libraries)
- **Libraries**: `abs libraries list`
- **Items**: `abs items list <LIBRARY_ID>`
- **Authors/Series**: `abs authors list`, `abs series list`
- **Metadata**: `abs metadata tags`, `abs metadata genres`

## Item Management

- **Get Details**: `abs items get <ITEM_ID>`
- **Update**: `abs items update <ITEM_ID> [OPTIONS]`
  - Options: `--title`, `--subtitle`, `--author`, `--narrator`, `--series`, `--genres`, `--tags`, `--year`
- **Bulk Update**: `abs items bulk-update "<ID1>,<ID2>" [OPTIONS]`
- **Match/Unmatch**: `abs items match <ITEM_ID>` or `abs items unmatch <ITEM_ID>`

## Library Maintenance

- **Scan**: `abs libraries scan <LIBRARY_ID>`
- **Full Rescan**: `abs libraries scan <LIBRARY_ID> --force`

## File Upload

- **Upload**: `abs upload <FILE_PATH> --library <LIB_ID> --title "<TITLE>" [OPTIONS]`
  - Options: `--author`, `--series`, `--folder`

## Advanced Usage

- **JSON Output**: Append `--json` to any command for raw data.
- **Completions**: `abs completion <SHELL>` (bash, zsh, etc.).
- **Help**: `abs <COMMAND> --help` for full parameter lists.

## Reference Documentation

- [Common Workflows](references/workflows.md)
- [Official Repository](https://github.com/LeeroyDing/audiobookshelf-cli)
