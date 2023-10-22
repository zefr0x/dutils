<h1 align=center>DUtils</h1>

Manage your default applications with ease using `DUtils`!

## Features

- Set default handler by extension or mime-type
- Intelligent mime type detection from files based on extension and content
- Open multiple files at once
- Set multiple handlers for mime/extension and use `rofi`/`dmenu` to pick one
- Wildcard support like `text/*`
- Automatically removes invalid/wrong `.desktop` entries from `mimeapps.list`
- Helper commands like `launch`, `get --json` for your scripting needs
- Unnecessarily fast (written in Rust)
- Single compiled binary with no dependencies

## Usage

```sh
# Open a file/URL
dutils open ~/.dotfiles/pacman/packages.txt
dutils open https://google.ca

# Set default handler for png files
dutils set .png feh.desktop

# Set wildcard handler for all text files
dutils set 'text/*' nvim.desktop

# Set default handler based on mime
dutils set application/pdf evince.desktop

# List default apps
dutils list

# Get the handler for a mime/extension
dutils get .png
feh.desktop

# Launch a handler with given path/URL
dutils launch x-scheme-handler/https -- https://google.ca
```

## Compared to `xdg-utils`

- Can open multiple files/URLs at once
- Can have multiple handlers and use rofi/dmenu to pick one at runtime
- Far easier to use with simple commands like `get`, `set`, `list`
- Can operate on extensions, **no need to look up or remember mime types**
  - useful for common tasks like setting a handler for png/docx/etc files
- Superb autocomplete (currently fish, zsh and bash), including mimes, extensions, and `.desktop` files
- Optional json output for scripting
- Properly supports `Terminal=true` entries

## Setting default terminal

Unfortunately, there isn't an XDG spec and thus a standardized way for `dutils` to get your default terminal emulator to run `Terminal=true` desktop entries. There was a proposal floating around a few years ago to use `x-scheme-handler/terminal` for this purpose. It seems to me the least worst option, compared to handling quirks of N+1 distros or using a dutils-specific config option.

Now if `x-scheme-handler/terminal` is present, `dutils` will use it.

Otherwise, `dutils` will:

1. Find an app with `TerminalEmulator` category
2. Set it as the default for `x-scheme-handler/terminal`
3. Send you a notification to let you know it guessed your terminal and provide instructions to change it if necessary

On the upside, `Terminal=true` entries will now work outside of interactive terminals, unlike `xdg-utils`.

## Setting multiple handlers

1. Open `~/.config/dutils/config.toml` and set `enable_selector = true`. Optionally, you can also tweak the `selector` to your selector command (using e.g. rofi or dmenu).

2. Add a second/third/whatever handler using `dutils add`, for example

```
dutils add x-scheme-handler/https firefox-developer-edition.desktop
```

3. When you open a URL, you will be prompted to select the desired application.

## Installation

TODO...

## Attribution

Originally forked from [chmln/handlr](https://github.com/chmln/handlr)
