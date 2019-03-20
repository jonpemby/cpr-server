# cpr-server
Simple iTerm2 coprocess server

## Build

You'll need Rust 1.3.x+ to build.

Simply run `cargo build` and then copy your build to `/usr/local/bin` (or wherever you keep utilities).

## Usage

You'll need a build of `coproc-server`.

Simply run this as an iTerm2 coprocess:

```sh
coproc-server
```

The server will start listening and any commands it recognizes will be sent to your iTerm2 window.
