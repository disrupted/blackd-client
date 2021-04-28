# blackd-client (WIP)

> Tiny HTTP client for the Black (`blackd`) Python code formatter

[Black](https://github.com/psf/black) is a brilliant, opinionated formatter for Python. However it can be quite slow when using an editor integration with format on save, since the process is cold-started every time you call it.

Luckily there's [blackd](https://black.readthedocs.io/en/stable/blackd.html) which is a small HTTP server that keeps the Black process running in the background so that it can be called directly without the lenghty startup time.

**blackd-client** is a simple helper that provides a single executable to communicate with Black, mainly for me to learn Rust.

If you're using Black (or writing Python code in general) I recommend you to check it out!

> this is very early stage and experimental. It's literally < 50 LOC I hacked together in one evening and I'll probably improve it as I learn more about Rust.

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

```sh
# TODO: provide binary
```

3. Start blackd daemon

```sh
blackd
```

or as service using Homebrew on macOS (preferred)

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

Normal `black --fast`:

```sh
❯ hyperfine 'cat subclean/core/parser.py | black --fast -'
  Time (mean ± σ):     296.8 ms ±  41.3 ms    [User: 228.7 ms, System: 51.1 ms]
  Range (min … max):   260.7 ms … 403.6 ms    10 runs
```

Using `blackd-client`

```sh
❯ hyperfine 'cat subclean/core/parser.py | blackd-client -'
  Time (mean ± σ):      32.2 ms ±   8.5 ms    [User: 6.5 ms, System: 7.2 ms]
  Range (min … max):    22.9 ms …  70.5 ms    61 runs
```

**Result:** blackd is almost 10x faster! :rocket:

## Neovim integration using EFM language server

[my configuration](https://github.com/disrupted/dotfiles/blob/master/.config/nvim/lua/efm/blackd.lua)

_TODO_ describe steps
