# Rtree project

Unix tree command but better written in the Rust programming language.

## Added features

> **Default behaviour:**  
> Display files as relative path from current directory recursively.

- Display files from a given root directory ( `--dir <directory>` or `-d <directory>` )  
- Display files in tree architecture ( `--tree` or `-t` )  *current issue with branching*
- Display files size ( `--size` or `-s` )  *current inaccuracy with file size values*
- Display files in a given order ( `--sorby <order>` )  
- Display hidden files ( `--all` or `-a` )  
- Display files recursively up to a maximum depth ( `--max-depth <depth>` or `-m <depth>` )  
- Display files only skipping over folders ( `--file-only` or `-f` )  

## Future features

> See [TODO.md](./TODO.md)

- Display statistics on files and/or given directory ( `--stats` )  
