# blackd-client (WIP)

> Tiny HTTP client for the Black (`blackd`) Python code formatter

[Black](https://github.com/psf/black) is a brilliant, opinionated formatter for Python. However it can be quite slow when using an editor integration with format on save, since the process is cold-started every time you call it.

Luckily there's [blackd](https://black.readthedocs.io/en/stable/usage_and_configuration/black_as_a_server.html) which is a small HTTP server that keeps the Black process running in the background so that it can be called directly without the lenghty startup time.

**blackd-client** is a simple helper that provides a single executable to communicate with Black, mainly for me to learn Rust.

If you're using Black (or writing Python code in general) I recommend you to check it out!

> this is very early stage and experimental. It's literally < 50 LOC I hacked together in one evening, and I'll probably improve it as I learn more about Rust.

## Install

1. Install Black

```sh
pip install black
```

or using Homebrew on macOS (preferred)

```sh
brew install black
```

2. Install blackd-client

- Download binary from https://github.com/disrupted/blackd-client/releases
- Rename to `blackd-client` and put it somewhere on your `PATH`

Alternatively if you have Rust toolchain installed:

```sh
cargo install blackd-client
```

3. Start blackd daemon

```sh
blackd
```

or as a launchd service using Homebrew on macOS (preferred)

```sh
sudo brew services start black
```

## Usage

pipe file contents to stdin, e.g.

```sh
cat main.py | blackd-client -
```

output is formatted using Black :sparkles:

## Benchmark comparison

Normal `black --fast`

```sh
❯ hyperfine 'cat subclean/core/parser.py | black --fast -'
  Time (mean ± σ):     296.8 ms ±  41.3 ms    [User: 228.7 ms, System: 51.1 ms]
  Range (min … max):   260.7 ms … 403.6 ms    10 runs
```

Using `blackd-client`

```sh
❯ hyperfine 'cat subclean/core/parser.py | blackd-client -'
  Time (mean ± σ):      23.7 ms ±   3.7 ms    [User: 2.7 ms, System: 4.8 ms]
  Range (min … max):    19.2 ms …  35.7 ms    84 runs
```

**Result:** blackd is more than 10x faster! :rocket:

## Neovim integration using EFM language server

[my configuration](https://github.com/disrupted/dotfiles/blob/master/.config/nvim/lua/efm/blackd.lua)

_TODO_ describe steps
