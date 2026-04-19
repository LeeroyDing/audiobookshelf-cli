# Common abs Workflows

## 1. Finding and Tagging a Book
First, search for the book:
```bash
abs search "Project Hail Mary"
```
Note the `ID`, then update tags:
```bash
abs items update <ID> --tags "Sci-Fi, Space"
```

## 2. Uploading and Organizing
Upload a file to a specific library:
```bash
abs upload "hail-mary.m4b" --library <LIB_ID> --title "Project Hail Mary" --author "Andy Weir"
```
Then trigger a scan to ensure it's picked up and processed:
```bash
abs libraries scan <LIB_ID>
```

## 3. Bulk Metadata Cleanup
List items in a library to find candidates:
```bash
abs items list <LIB_ID>
```
Update multiple items at once:
```bash
abs items bulk-update "ID1,ID2,ID3" --genres "Science Fiction" --year 2021
```

## 4. Scripting with JSON
Use `jq` to extract specific info:
```bash
abs libraries list --json | jq '.[].name'
```
