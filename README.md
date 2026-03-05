[![release](https://img.shields.io/badge/v0.0.7-a6e3a1?style=for-the-badge&labelColor=1e1e2e&logoColor=a6e3a1&label=release)](https://github.com/AndyLocks/string-pool/releases/tag/stringp-v0.0.7)
[![gpl](https://img.shields.io/badge/gpl-f9e2af?style=for-the-badge&label=license&labelColor=1e1e2e)](https://github.com/AndyLocks/string-pool/blob/master/LICENSE)
[![gpl](https://img.shields.io/badge/AUR-89b4fa?style=for-the-badge&labelColor=1e1e2e&logo=archlinux&logoColor=cdd6f4)](https://aur.archlinux.org/packages/stringp)

A "key-value" utility for UNIX-like systems.

Allows to conveniently store values and retrieve them by key. `stringp` considers a directory to be a key-value storage, containing files with names, where a filename is a key, and a content of a file is a value.

# Usage

## Add value

Creates a new file and writes everything that was passed to `stdin` into it. If `stdin` is empty, the new file will also be empty.

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

---

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

## Key

The `key` command makes it easier to generate arguments for the `get` command:

```bash
stringp key arg value
```

This command will generate the following output:

```
{"arg":"value"}
```

This command can be used in pipelines to add more keys and values:

```bash
stringp k arg1 value1 | stringp k arg2 value2 
```

This command will generate the following output:

```
{"arg1":"value1","arg2":"value2"}
```

The following two examples are completely equivalent:

```bash
stringp k arg1 value1 | stringp k arg2 value2 | stringp g key
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

## `list` command

Outputs a list of existing keys:

```bash
stringp list
stringp keys
stringp l
```

## `edit` command

Opens a file named by the key in storage directory with the editor from `$EDITOR`, if it exists. If not, it tries to open `editor`, `nano`, `vim` or `vi`.

```bash
stringp edit key
EDITOR=nvim stringp edit key
```

# Directory rewriting

By changing the directory, you can change the "database" and have the one you need for each case. Hierarchy:

1. `--dir`
2. Environment variable `STRING_POOL_DIR`
3. Use `~/.local/share/string-pool` directory by default, if exists

```bash
stringp get key --dir .
stringp get --dir . key
STRING_POOL_DIR="." stringp get key
stringp get key #~/.local/share/string-pool by default
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
stringp k src $(fzf) | stringp g latex-picture | wl-copy
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

```bash
git clone "$(stringp get server-user)@$(stringp get server-ip):/path/to/gitrepo/$(gum input --placeholder='Repo name')"
```

# Build

```bash
git clone https://github.com/AndyLocks/string-pool ; cd string-pool
cargo build --release
```

The binary can be found in `./target/release` directory.

## Install (requires su)

### Bin

```bash
install -Dm755 ./target/release/stringp /usr/bin/stringp
```

### Man page

```bash
install -Dm644 ./stringp.1.gz /usr/share/man/man1/stringp.1.gz
```

### Auto completion

```bash
mkdir -p "/usr/share/bash-completion/completions"
mkdir -p "/usr/share/zsh/site-functions"
mkdir -p "/usr/share/fish/vendor_completions.d"

"/usr/bin/stringp" completions bash > "/usr/share/bash-completion/completions/stringp"
"/usr/bin/stringp" completions zsh > "/usr/share/zsh/site-functions/_stringp"
"/usr/bin/stringp" completions fish > "/usr/share/fish/vendor_completions.d/stringp.fish"
```

### License

```bash
install -Dm644 LICENSE "/usr/share/licenses/stringp/LICENSE"
```

