# tts-cli

Command line utility to automatically read from clipboard.

You can only exit with `Ctrl-C`. I want to keep this project very simple.

To avoid errors you can lower the FPS const.

The binary is called `tts` instead of `tts-cli` for brevity.

## Windows
Add voice packs or change the default voice in your system settings.
### Install
- cargo install tts-cli

### Build
- cargo run --release

## Linux
These voices are not as good, sadly.
- sudo apt install llvm-dev libclang-dev clang
  - https://rust-lang.github.io/rust-bindgen/requirements.html#debian-based-linuxes
- sudo apt install speech-dispatcher

then one of the following based on `speech-dispatcher -v`
  ### Install
  - cargo install tts-cli
  - cargo install tts-cli --no-default-features --features 10
  - cargo install tts-cli --no-default-features --features 9
  ### Build
  - cargo run --release
  - cargo run --release --no-default-features --features 10
  - cargo run --release --no-default-features --features 9
