[![release](https://img.shields.io/badge/v0.0.3-a6e3a1?style=for-the-badge&labelColor=1e1e2e&logoColor=a6e3a1&label=release)](https://github.com/AndyLocks/string-pool/releases/tag/stringp-v0.0.3)
[![gpl](https://img.shields.io/badge/gpl-f9e2af?style=for-the-badge&label=license&labelColor=1e1e2e)](https://github.com/AndyLocks/string-pool/blob/master/LICENSE)
[![gpl](https://img.shields.io/badge/AUR-89b4fa?style=for-the-badge&labelColor=1e1e2e&logo=archlinux&logoColor=cdd6f4)](https://aur.archlinux.org/packages/stringp)

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

```
You can also add arguments to your files!

For example: ${arg1}
Or: ${arg2}
```

Arguments are passed via stdin in JSON format:

```bash
echo '{"arg1":"value1","arg2":"value2"}' | stringp get key
```

Then you will get the following output:

```
You can also add arguments to your files!

For example: value1
Or: value2
```

## Build

The `build` command makes it easier to generate arguments for the `get` command:

```bash
stringp build arg value
```

This command will generate the following output:

```
{"arg":"value"}
```

This command can be used in pipelines to add more keys and values:

```bash
stringp b arg1 value1 | stringp b arg2 value2 
```

This command will generate the following output:

```
{"arg1":"value1","arg2":"value2"}
```

The following two examples are completely equivalent:

```bash
stringp b arg1 value1 | stringp b arg2 value2 | stringp g key
```

```bash
echo '{"arg1":"value1","arg2":"value2"}' | stringp get key
```

If you need to leave a string like `${string}` untouched in a file, there are several ways to do this:

1. Do not pass an argument with that name.
2. Replace `${string}` with `$\{string}` and use the `-s` flag, which will replace `\{` with `{`.

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

## Templates

For example, we can add this template for LaTeX:

```latex
\begin{figure}[H]
    \centering
    \includegraphics[width=0.6\textwidth]{${src}}
\end{figure}
```

The following command will display the [fzf](https://github.com/junegunn/fzf) menu, where you can select the path to the image. This path will then be inserted into the template, and the template will be copied to the clipboard:

```
stringp b src $(fzf) | stringp g latex-picture | wl-copy
```

You can also make more complex templates, for example, here is one of mine for pandoc:

```
---
title: "**${title}**"
author: "${name}"
date: "${date}"
lang: "ru"
fontsize: 12pt
geometry: a4paper
mainfont: "LiberationSans"
header-includes: |
  \usepackage{xcolor}
  \usepackage{float}
  \usepackage{hyperref}
  \geometry{left=2cm,right=2cm,top=2cm,bottom=2cm}
  \hypersetup{
    colorlinks=true,
    linkcolor=blue,
    urlcolor=blue,
    bookmarksopen=true,
    bookmarksopenlevel=10
  }
  \usepackage{graphicx}
  \usepackage{multicol}
  \setlength{\columnsep}{1.5cm}
output: pdf_document
pdf-engine: xelatex
---
```

## Constants

You can store links, file paths, or MAC addresses and use them in your scripts, and easily change the values if necessary.

# Build

```bash
git clone https://github.com/AndyLocks/string-pool ; cd string-pool
cargo build --release
```

The binary can be found in `./target/release` directory.
