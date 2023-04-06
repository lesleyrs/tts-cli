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
- sudo apt install speech_dispatcher

then based on `speech-dispatcher -v`
- ### Install
    - cargo install tts-cli --features tts/speech_dispatcher_0_11
    - cargo install tts-cli --features tts/speech_dispatcher_0_10
    - cargo install tts-cli --features tts/speech_dispatcher_0_09
- ### Build
    - cargo run --release --features tts/speech_dispatcher_0_11
    - cargo run --release --features tts/speech_dispatcher_0_10
    - cargo run --release --features tts/speech_dispatcher_0_09
