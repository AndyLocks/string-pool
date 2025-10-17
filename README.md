[![release](https://img.shields.io/badge/v0.0.1-a6e3a1?style=for-the-badge&labelColor=1e1e2e&logoColor=a6e3a1&label=release)]()

A "key-value" utility for UNIX-like systems.

Allows to conveniently store values and retrieve them by key.

# Configuration

The configuration file `.config/string-pool/config.toml` is created automatically and looks like this by default:

```toml
directory = "/home/yourname/.local/share/string-pool"
```

The `.local/share/string-pool` directory contains files whose names are keys and whose contents are values. By changing the names or contents of these files, you will change the keys and values for `stringp`, so you don't need to use the `remove` and `add` commands — you can change the files directly. Using the `remove` and `add` commands only makes sense when you want to hide directory information from the calling side.

# Usage

## Add value

Creates a new file and writes everything that was passed to `stdin` into it.

```bash
cat content.txt | stringp add 'key'
cat content.txt | stringp a 'key'
```

In this example, a file named `key` will be created with the contents of the file `content.txt`.

## Get value

Outputs the value by key:

```bash
stringp get key
stringp g key
```

If such a key does not exist, nothing will be displayed, and the program will terminate with code 1.

## Remove value

Delete file by key.

```bash
stringp remove key
stringp rm key
```

## List

Outputs a list of existing keys:

```bash
stringp list
stringp keys
stringp l
```

# Directory rewriting

The directory used is also influenced by environment variables and the `--dir` flag. By changing the directory, you can change the "database" and have the one you need for each case. Hierarchy:

1. `--dir`
2. env (`STRING_POOL_DIR`)
3. `.config/string-pool/config.toml`

```bash
stringp get key1 --dir .
STRING_POOL_DIR="." stringp get key2
stringp get key3
```

# Examples of use

## [fzf](https://github.com/junegunn/fzf)

You can use [fzf](https://github.com/junegunn/fzf) or similar tools to find and select the desired key and get the value:

```bash
stringp get $(stringp keys | fzf)
```

## [Rofi](https://github.com/davatorium/rofi) or [Wofi](https://github.com/SimplyCEO/wofi)

Another similar example: You can conveniently display all available keys using [wofi](https://github.com/SimplyCEO/wofi) or [rofi](https://github.com/davatorium/rofi), find and select the one you need, and then, for example, copy the value to the clipboard.

```bash
stringp get "$(stringp keys | wofi --dmenu)" | wl-copy
```

This can be useful if you often write in markup languages such as LaTeX and frequently forget certain template elements. You can also bind this command to any key combination, depending on your shell.

## Constants

You can store links, file paths, or MAC addresses and use them in your scripts, and easily change the values if necessary.
