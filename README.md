# tts-cli

Command line utility to automatically read from clipboard.

## Windows
- cargo run

Add voice packs or change the default voice in your system settings.

## Linux
These voices are not as good, sadly.
- sudo apt install llvm-dev libclang-dev clang
  - https://rust-lang.github.io/rust-bindgen/requirements.html#debian-based-linuxes
- sudo apt install speech_dispatcher
- cargo run
  - if it fails, use `cargo add tts --no-default-features --features speech_dispatcher_0_10` first.
